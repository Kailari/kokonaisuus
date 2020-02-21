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

pub trait StorageTuple<'a> {
    type Accessors;
    type Values;
    type Iterators: IteratorTuple<'a, Self::Accessors, Item=Self::Values>;

    fn iterator(&'a mut self) -> TupleIter<'a, Self::Accessors, Self::Iterators, Self::Values>;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// INSANITY STARTS HERE
////////////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! define_storage_tuple {
    ($(($i:tt, $type_name:ident)),*) => {
        // Implement `StorageTuple` for arbitrary tuples which members all implement `Fetch`
        // A lot of things happen on these line(s):
        //  1.  impl declares all type_names as type parameters
        //  2.  we declare the implementation for tuples with all type names as members (this can be
        //      done as we declared them in step 1)
        impl<'a, $($type_name),*> StorageTuple<'a> for ($($type_name),*)
            // ...step 3: Declare that all of the type parameters implement `Fetch`
            where $($type_name: Fetch<'a>),*
        {
            // Now, we can use the type params to declare the helper types. This is possible as we
            // have forced the type params to implement `Fetch`.
            // TODO: Accessors is essentially Self?
            type Accessors = ($($type_name),*,);            // Expands to (A, B, ...)
            type Values = ($($type_name::Item),*,);         // Expands to (A::Item, B::Item, ...)
            type Iterators = ($($type_name::Iterator),*,);  // Expands to (A::Iterator, B::Iterator, ...)

            fn iterator(&'a mut self) -> TupleIter<'a, Self::Accessors, Self::Iterators, Self::Values> {
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
}

define_storage_tuple!((0, A), (1, B));

/*impl<'a, A, B> StorageTuple<'a> for (A, B)
    where A: Fetch<'a>,
          B: Fetch<'a> {
    type Accessors = (A, B);
    type Values = (A::Item, B::Item);
    type Iterators = (A::Iterator, B::Iterator);

    fn iterator(&'a mut self) -> TupleIter<'a, Self::Accessors, Self::Iterators, Self::Values> {
        TupleIter {
            iterators: (self.0.fetch_iter(), self.1.fetch_iter()),
            _values: PhantomData,
            _access: PhantomData,
        }
    }
}*/

impl<'a, A, B> IteratorTuple<'a, (A, B)> for (A::Iterator, B::Iterator)
    where A: Fetch<'a>,
          B: Fetch<'a> {
    type Item = (A::Item, B::Item);

    fn next_all(&mut self) -> Option<Self::Item> {
        match (self.0.next(), self.1.next()) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        }
    }
}
