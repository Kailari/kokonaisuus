use std::slice::Iter;
use std::sync::RwLockReadGuard;

use crate::storage::{ComponentStorage, Fetch};

/// Provides read-only access to a component storage. Implements [`Fetch`](../trait.Fetch.html) to
/// provide immutable iterators to [`StorageTuple`s](../storage_tuple/trait.StorageTuple.html)
pub struct Read<'a, C> {
    storage: &'a ComponentStorage<C>,
    guard: Option<RwLockReadGuard<'a, Vec<C>>>,
}

impl<'a, C> Read<'a, C> {
    pub fn new(storage: &'a ComponentStorage<C>) -> Read<'a, C> {
        Read { storage, guard: None }
    }
}

impl<'a, C> Fetch<'a> for Read<'a, C> {
    type Component = &'a C;
    type Iterator = Iter<'a, C>;
    type Guard = RwLockReadGuard<'a, Vec<C>>;

    fn fetch_accessor(&'a mut self) -> &Self::Guard {
        self.guard = Some(self.storage.fetch_for_reading());
        // FIXME: Returning a ref to the guard might be unnecessary
        self.guard.as_ref().unwrap()
    }
}

impl<'a, 'b: 'a, C: 'a> IntoIterator for &'b Read<'a, C> {
    type Item = <Read<'a, C> as Fetch<'a>>::Component;
    type IntoIter = <Read<'a, C> as Fetch<'a>>::Iterator;

    fn into_iter(self) -> Self::IntoIter {
        self.guard.as_ref().unwrap().iter()
    }
}
