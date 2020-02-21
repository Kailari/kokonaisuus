use crate::components::{AmountComponent, ValueComponent};
use crate::systems::System;
use crate::tuple_iter::StorageTuple;

/// System for printing the value and increment pairs.
pub struct PrintSystem;

impl<'a> System<'a> for PrintSystem {
    type Data = (&'a mut Vec<ValueComponent>,
                 &'a mut Vec<AmountComponent>);

    // This is the compacted/improved way of doing things. See adder.rs for more readable version
    // without trait magic.
    fn tick(&self, (values, amounts): Self::Data) {
        // At the top of the file we have `use::tuple_iter::IterableTuple`, which imports the trait
        // and its `impl`-blocks. As we have implemented the `IterableTuple`-trait for arbitrary
        // tuples containing two vectors, compiler recognizes that the tuple we create here has
        // the `IterableTuple`-trait, allowing us to call `.iterator()` on it.
        //
        // Then, behind the scenes, our trait implementation in tuple_iter.rs simply creates
        // iterators for the vectors (values and amounts) and wraps the iterators into a wrapper
        // `IteratorTuple`. This tuple iterator is then supplied to `for (...) in iterator_tuple`
        // which iterates it.
        //
        // The `IteratorTuple` implements `next()` so that it produces tuples of components in the
        // supplied vectors by advancing both of the wrapped iterators on each call. On the left of
        // the for-expression here, we just de-structure that tuple into the components so that they
        // can be used in the loop.
        for (value, amount) in (values, amounts).iterator() {
            println!("Value: {:?}, Amount: {:?}", value.value, amount.amount)
        }
    }
}
