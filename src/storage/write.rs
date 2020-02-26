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

impl<'a, C> StorageLock for Write<'a, C> {
    type Accessor = StorageWriter<'a, C>;

    fn claim(self) -> Self::Accessor {
        StorageWriter { guard: self.storage.components.write().unwrap() }
    }
}

pub struct StorageWriter<'a, C> {
    pub guard: RwLockWriteGuard<'a, Vec<C>>,
}
