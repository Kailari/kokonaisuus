use std::slice::IterMut;
use std::sync::RwLockWriteGuard;

use crate::storage::{ComponentStorage, StorageLock};

/// Provides read/write access to a component storage. Implements [`Fetch`](../trait.Fetch.html) to
/// provide mutable iterators to [`StorageTuple`s](../storage_tuple/trait.StorageTuple.html)
pub struct Write<'a, C> {
    storage: &'a mut ComponentStorage<C>,
}

impl<'a, C> Write<'a, C> {
    pub fn new(storage: &'a mut ComponentStorage<C>) -> Write<'a, C> {
        Write { storage }
    }
}

impl<'a: 'b, 'b, C> StorageLock<'a, 'b> for &'b Write<'a, C> {
    type Accessor = StorageWriter<'b, C>;

    fn claim(self) -> Self::Accessor {
        StorageWriter { guard: self.storage.components.write().unwrap() }
    }
}

pub struct StorageWriter<'a, C> {
    pub guard: RwLockWriteGuard<'a, Vec<C>>,
}

impl<'a, 'b, A> IntoIterator for &'a mut StorageWriter<'b, A> {
    type Item = &'a mut A;
    type IntoIter = IterMut<'a, A>;

    fn into_iter(self) -> Self::IntoIter {
        self.guard.iter_mut()
    }
}
