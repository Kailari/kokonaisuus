use crate::components::{AmountComponent, ValueComponent};
use crate::storage::{Read, StorageLock, Write};
use crate::systems::System;

/// System for incrementing values by their respective increments.
pub struct AdderSystem;

pub struct TupleIter<T: IteratorTuple> {
    iterators: T,
}

impl<T: IteratorTuple> Iterator for TupleIter<T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterators.next_all()
    }
}

pub trait IteratorTuple: Sized {
    type Item;

    fn next_all(&mut self) -> Option<Self::Item>;

    fn iterator(self) -> TupleIter<Self>;
}

impl<'a, A, B> IteratorTuple for (A, B)
    where A: Iterator,
          B: Iterator
{
    type Item = (A::Item, B::Item);

    fn next_all(&mut self) -> Option<Self::Item> {
        match (self.0.next(), self.1.next()) {
            (Some(value), Some(amount)) => Some((value, amount)),
            _ => None,
        }
    }

    fn iterator(self) -> TupleIter<Self> {
        TupleIter { iterators: self }
    }
}

impl<'a> System<'a> for AdderSystem {
    type Data = (Write<'a, ValueComponent>,
                 Read<'a, AmountComponent>);

    fn tick(&self, (values, amounts): Self::Data) {
        for (value, amount) in (values.claim().guard.iter_mut(), amounts.claim().guard.iter()).iterator() {
            value.value += amount.amount;
        }
    }
}
