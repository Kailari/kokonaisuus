use std::marker::PhantomData;
use std::slice::{Iter, IterMut};

use crate::components::{ComponentStorage, Storage};

pub trait Fetch<'a> {
    type Item;
    type Iterator;
    type Storage;

    fn fetch(&'a mut self) -> Self::Iterator;
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
    type Storage = &'a mut ComponentStorage<C>;

    fn fetch(&'a mut self) -> Self::Iterator {
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
    type Storage = &'a ComponentStorage<C>;

    fn fetch(&'a mut self) -> Self::Iterator {
        self.storage.fetch_for_reading().iter()
    }
}

pub struct TupleIter<I, V> {
    iterators: I,
    values_phantom: PhantomData<V>,
}

impl<'a, A, B> Iterator for TupleIter<(IterMut<'a, A>, Iter<'a, B>), (&'a mut A, &'a B)> {
    type Item = (&'a mut A, &'a B);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.iterators.0.next(), self.iterators.1.next()) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        }
    }
}


pub trait StorageTuple<'a> {
    type Iterators;
    type Storages;
    type Values;

    fn iterator(&'a mut self) -> TupleIter<Self::Iterators, Self::Values>;
}

impl<'a, A, B> StorageTuple<'a> for (A, B)
    where A: Fetch<'a>,
          B: Fetch<'a> {
    type Iterators = (<A as Fetch<'a>>::Iterator,
                      <B as Fetch<'a>>::Iterator);
    type Storages = (<A as Fetch<'a>>::Storage, <B as Fetch<'a>>::Storage);
    type Values = (<A as Fetch<'a>>::Item, <B as Fetch<'a>>::Item);

    fn iterator(&'a mut self) -> TupleIter<Self::Iterators, Self::Values> {
        TupleIter {
            iterators: (self.0.fetch(), self.1.fetch()),
            values_phantom: PhantomData,
        }
    }
}
