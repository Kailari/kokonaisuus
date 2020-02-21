use std::marker::PhantomData;

use crate::storage::Fetch;

pub trait IteratorTuple<'a, A> {
    type Item;

    fn next_all(&mut self) -> Option<Self::Item>;
}

pub struct TupleIter<'a, A, I: IteratorTuple<'a, A, Item=V>, V> {
    iterators: I,
    _access: PhantomData<A>,
    _values: PhantomData<&'a V>,
}

impl<'a, A, I: IteratorTuple<'a, A, Item=V>, V> Iterator for TupleIter<'a, A, I, V> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterators.next_all()
    }
}

// Must specify `Sized` as supertrait as `Self` is directly used as type param
pub trait StorageTuple<'a>: Sized {
    type Values;
    type Iterators: IteratorTuple<'a, Self, Item=Self::Values>;

    fn iterator(&'a mut self) -> TupleIter<'a, Self, Self::Iterators, Self::Values>;
}


////////////////////////////////////////////////////////////////////////////////////////////////////
// INSANITY STARTS HERE - "Ken tästä käy, saa kaiken toivon heittää"
////////////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! define_tuple {
    // Base case, defines both tuple implementations for a n-dimensional tuple
    ($(($i:tt, $item_name:ident, $type_name:ident)),*) => {
        define_tuple! { stor $(($i, $type_name)),* }
        define_tuple! { iter $(($i, $item_name, $type_name)),* }
    };

    // Implement `StorageTuple` for arbitrary tuples which members all implement `Fetch`
    (stor $(($i:tt, $type_name:ident)),*) => {
        // A lot of things happen on these line(s):
        //  1.  impl declares all type_names as type parameters
        //  2.  we declare the implementation for tuples with all type names as members (this can be
        //      done as we declared them in step 1)
        impl<'a, $($type_name),*> StorageTuple<'a> for ($($type_name),*,)
            // ...step 3: Declare that all of the type parameters implement `Fetch`
            where $($type_name: Fetch<'a>),*
        {
            // Now, we can use the type params to declare the helper types. This is possible as we
            // have forced the type params to implement `Fetch`.
            type Values = ($($type_name::Item),*,);         // Expands to (A::Item, B::Item, ...)
            type Iterators = ($($type_name::Iterator),*,);  // Expands to (A::Iterator, B::Iterator, ...)

            fn iterator(&'a mut self) -> TupleIter<'a, Self, Self::Iterators, Self::Values> {
                TupleIter {
                    // Last thing left to do is to fetch an iterator for all of the wrapped storage
                    // accessors. As the implementation is on a tuple which contains the accessors,
                    // just use tuple indexing to get the accessors for fetching.
                    iterators: ($(self.$i.fetch_iter()),*,),
                    _values: PhantomData,
                    _access: PhantomData,
                }
            }
        }
    };

    // Implement `IteratorTuple` for tuples constructed during `StorageTuple::iterator()`
    (iter $(($i:tt, $item_name:ident, $type_name:ident)),*) => {
        // Very similarly to how storage tuples are declared, we expand type names to type params.
        // Here, each expansion on their own line, what this expands to:
        impl<'a, $($type_name),*>               // Expands to `impl<'a, A, B, ...>`
        IteratorTuple<'a, ($($type_name),*,)>    // Expands to `IteratorTuple<'a, A, B, ...>`
        for ($($type_name::Iterator),*,)        // Expands to `for (A::Iterator, B::Iterator, ...)`
        where $($type_name: Fetch<'a>),*        // Expands to `where A: Fetch<'a>, B: Fetch<'a>, ...`
        {
            // Again, define an associated type for easier time with output types
            type Item = ($($type_name::Item),*,); // Expands to (A::Item, B::Item, ...)

            fn next_all(&mut self) -> Option<Self::Item> {
                // This Expands to `match (self.0.next(), self.1.next(), ...) {`, which essentially
                //  1. calls `Iterator::next()` on all wrapped iterators
                //  2. constructs a tuple of optionals of the values produced by step 1
                //  3. matches against that tuple to ensure all values exist
                match ($( self.$i.next() ),*,) {
                    ($( Some($item_name) ),*,) => Some(($( $item_name ),*,)),
                    _ => None,
                }
            }
        }
    };
}

define_tuple! { (0, a, A) }
define_tuple! { (0, a, A), (1, b, B) }
