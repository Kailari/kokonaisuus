use std::sync::RwLockReadGuard;

use crate::storage::{ComponentStorage, StorageLock};
use std::slice::Iter;

/// Provides read-only access to a component storage. Implements [`Fetch`](../trait.Fetch.html) to
/// provide immutable iterators to [`StorageTuple`s](../storage_tuple/trait.StorageTuple.html)
pub struct Read<'a, C> {
    storage: &'a ComponentStorage<C>,
}

impl<'a, C> Read<'a, C> {
    pub fn new(storage: &'a ComponentStorage<C>) -> Read<'a, C> {
        Read { storage }
    }
}

impl<'a: 'b, 'b, C> StorageLock<'a, 'b> for Read<'a, C> {
    type Accessor = StorageReader<'b, C>;

    fn claim(&'b self) -> Self::Accessor {
        StorageReader { guard: self.storage.components.read().unwrap() }
    }
}

pub struct StorageReader<'a, C> {
    pub guard: RwLockReadGuard<'a, Vec<C>>,
}


impl<'a, 'b, A> IntoIterator for &'a StorageReader<'b, A> {
    type Item = &'a A;
    type IntoIter = Iter<'a, A>;

    fn into_iter(self) -> Self::IntoIter {
        self.guard.iter()
    }
}
