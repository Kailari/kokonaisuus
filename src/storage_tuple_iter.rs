use std::marker::PhantomData;
use std::slice::{Iter, IterMut};

use crate::components::{ComponentStorage, Storage};

pub trait Fetch<'a> {
    type Item;
    type Iterator: Iterator<Item=Self::Item>;

    fn fetch_iter(&'a mut self) -> Self::Iterator;
}

pub struct Write<'a, C> {
    storage: &'a mut ComponentStorage<C>,
}

impl<'a, C> Write<'a, C> {
    pub fn new(storage: &'a mut ComponentStorage<C>) -> Write<'a, C> {
        Write { storage }
    }
}

impl<'a, C> Fetch<'a> for Write<'a, C> {
    type Item = &'a mut C;
    type Iterator = IterMut<'a, C>;

    fn fetch_iter(&'a mut self) -> Self::Iterator {
        self.storage.fetch_for_writing().iter_mut()
    }
}

pub struct Read<'a, C> {
    storage: &'a ComponentStorage<C>,
}

impl<'a, C> Read<'a, C> {
    pub fn new(storage: &'a ComponentStorage<C>) -> Read<'a, C> {
        Read { storage }
    }
}

impl<'a, C> Fetch<'a> for Read<'a, C> {
    type Item = &'a C;
    type Iterator = Iter<'a, C>;

    fn fetch_iter(&'a mut self) -> Self::Iterator {
        self.storage.fetch_for_reading().iter()
    }
}

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
    type Values = (<A as Fetch<'a>>::Item,
                   <B as Fetch<'a>>::Item);
    type Iterators = (<A as Fetch<'a>>::Iterator,
                      <B as Fetch<'a>>::Iterator);

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
