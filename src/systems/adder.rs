use std::slice::{Iter, IterMut};

use crate::components::{AmountComponent, ValueComponent};
use crate::storage::{Read, StorageLock, StorageReader, StorageWriter, Write, IntoIteratorTuple};
use crate::systems::System;

/// System for incrementing values by their respective increments.
pub struct AdderSystem;

impl<'a, 'b, A> IntoIterator for &'a StorageReader<'b, A> {
    type Item = &'a A;
    type IntoIter = Iter<'a, A>;

    fn into_iter(self) -> Self::IntoIter {
        self.guard.iter()
    }
}

impl<'a, 'b, A> IntoIterator for &'a mut StorageWriter<'b, A> {
    type Item = &'a mut A;
    type IntoIter = IterMut<'a, A>;

    fn into_iter(self) -> Self::IntoIter {
        self.guard.iter_mut()
    }
}

impl<'a> System<'a> for AdderSystem {
    type Data = (Write<'a, ValueComponent>,
                 Read<'a, AmountComponent>);

    fn tick(&self, data: Self::Data) {
        let (mut values, amounts) = data.claim();

        for (value, amount) in (&mut values, &amounts).iterator() {
            value.value += amount.amount;
        }
    }
}
