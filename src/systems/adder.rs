use crate::components::{AmountComponent, ValueComponent};
use crate::storage::{IntoIteratorTuple, Read, StorageLock, Write};
use crate::systems::System;

/// System for incrementing values by their respective increments.
pub struct AdderSystem;

impl<'a> System<'a> for AdderSystem {
    type Data = (Write<'a, ValueComponent>,
                 Read<'a, AmountComponent>);

    fn tick(&self, (value_lock, amount_lock): Self::Data) {
        // Wait a minute? What is this seemingly unnecessary block here? Glad you didn't aks, I'll
        // explain anyway!
        //
        // Blocks are useful for scoping lifetimes. "What?" This block here allows us to tell the
        // compiler explicitly that we want the variables defined within it to be dropped once the
        // block ends. This has implications when dealing with objects with well-defined lifetimes
        // or more complex constraints on them.
        //
        // In practice, what this block does is quite simple: We use the block to tell the compiler
        // when we are done with the data manipulation and we would like to release the component
        // storages for other systems to use.
        //  1. `let (values, amounts) = (...).claim();` acquires the locks on component storages
        //  2. when the block ends (`}`), lifetime of `values` and `amounts` ends and they are
        //     dropped. As they are structs holding the locks in their fields, that data gets
        //     released, releasing the lock.
        //  *. (as additional information on "2.": We use mechanism called `RwLock` for locking the
        //      storages. `RwLock` wraps a vector and provides methods for claiming read/write
        //      "guards" which are used to access the vector concurrently. Dropping such guard has
        //      the effect of releasing the lock, which in practice is what's going on here. For
        //      more info, see `storage/mod.rs`)
        {
            // Claim locks on the storages. Create tuple of refs to locks and call `.claim()`.
            // Powered by the black magic and awesomeness of traits!
            //
            // More of a fun fact than anything particularly useful:
            //  -   Use refs instead of values. Why? If `value_lock` and `amount_lock` were passed
            //      value here, their ownership would move to the tuple being created. This in turn
            //      is a temporary variable, which gets dropped as soon as `.claim()` ends
            //      (`.claim()` consumes the ownership but does not return or pass it along). This,
            //      obviously, is a compiler error as we would have dropped the temporary tuple
            //      before the assignment and destructuring could happen.
            //
            // What are we trying to do?
            //  1.  We want to access the component storages, FROM MULTIPLE SYSTEMS, POSSIBLY BEING
            //      EXECUTED IN PARALLEL. Multiple simultaneous reads are permitted, but only one
            //      system can write to the storage at a time. No reads are allowed while writing.
            //      This requires locking mechanism. (We use `Arc<RwLock<Vec<ComponentType>>>` to
            //      achieve this behind the scenes. See `storage/mod.rs` for more info)
            //  2.  So, we need to somehow claim the appropriate locks on the storages. We have the
            //      `value_lock` and `amount_lock` for doing just that. They are of types `Read` and
            //      `Write` for read-only and read-write-access, respectively. Those types implement
            //      the `StorageLock` trait, which provides `.claim()` -function for claiming the
            //      lock. This allows us to write:
            //
            // let mut values = &value_lock.claim();
            // let amounts = &amount_lock.claim();
            //
            //  3.  Two lines!? Outrageous! We must do better than that! Let's use tuples:
            //
            // let (mut values, amounts) = ((&value_lock).claim(), (&amount_lock).claim());
            //
            //  4.  Holy-**** that's ugly. Workaround: Implement `StorageLock` on tuples containing
            //      only `StorageLocks` (this happens via macro in `storage_tuple.rs`) so that
            //      calling `.claim()` on that tuple calls claim on all its elements, returning the
            //      return values from those calls as a tuple. This allows us to write:
            let (mut values, amounts) = (&value_lock, &amount_lock).claim();

            // Now that we have a tuple of storage "accessors" (`StorageReader` or `StorageWriter`),
            // we somehow need to iterate over them. The next line is quite magical, it does just
            // that; converts the accessors into iterators and tuple of iterators into a single
            // `TupleIter`, all in a single line. There is A LOT going on this line.
            //      1.  Let's start with some pseudocode. What we want to do is to be able to call
            //          some method on a tuple of storage accessors and get an iterator out. This
            //          looks something like:
            //
            //  for (mut value, amount) in (values, amounts).iterator() { ...
            //
            //      2.  Now, we do not want to consume the accessors (might want to do multiple
            //          separate loops within a single system) so we need to use references instead
            //          of passing the values directly:
            //
            //  for (mut value, amount) in (&mut values, &amounts).iterator() { ...
            //
            //      3.  OK! Now, how to get this thing to actually compile? At this point, we make
            //          the task easier by solving part of it beforehand. In `storage_tuple.rs` we
            //          define a trait for *tuples of iterators*, which allows us to create a
            //          compound iterator over the tuple of other iterators. (In other words, given
            //          a tuple containing only iterators, provide `.iterator()` which creates a
            //          "wrapper" iterator for iterating all of the child iterators at once). Now,
            //          this breaks the problem down to
            //
            //  for (mut value, amount) in (value_iter, amount_iter).iterator() { ...
            //
            //      2.  But how do we obtain the iterator for some random accessor-struct which
            //          obviously is not iterable? Standard library to the rescue; `IntoIterator`
            //          is a trait which allows us to provide implementation for converting a struct
            //          into an iterator. In this case, we implement that *for references to*
            //          storage accessors (see `write.rs` and `read.rs`). This allows us to write:
            //
            //  for (...) in ((&mut values).into_iter(), (&amounts).into_iter()).iterator() { ...
            //
            //      3.  Now we're talking! This actually compiles already, but syntax is horrible.
            //          There is a trick involving the fact that anything which implements a trait
            //          `IntoIterator` is effectively treated as an iterator (as it can be converted
            //          into one using `a.into_iter()`) Thus, the compiler is smart enough to place
            //          the `.into_iter()` calls auto-magically for us, so that we can write:
            for (value, amount) in (&mut values, &amounts).iterator() {
                value.value += amount.amount;
            }
        }

        // Here, we do not need the block as we do not use the locks again before the method ends.
        // The end of the method has the same effect on these locks as the extra block above does
        // for the values and amounts claimed within it.
        // {

        // Notable thing here: We are re-using the same `value_lock`. This is possible before the
        // previous block dropped all its references to it, returning it back to the outside of
        // the block.
        let (mut values, ) = (&value_lock, ).claim();

        for (value, ) in (&mut values, ).iterator() {
            println!("Value after printing: {}", value.value)
        }
        // }
    }
    // Lifetime of this method invocation ends, dropping everything which is still owned here. That
    // causes the remaining storage locks to be freed.
}
