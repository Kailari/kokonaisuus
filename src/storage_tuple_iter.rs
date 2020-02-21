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

impl<'a, A, B> StorageTuple<'a> for (A, B)
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
}

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
