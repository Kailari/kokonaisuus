use std::slice::IterMut;
use std::sync::RwLockWriteGuard;

use crate::storage::{ComponentStorage, Fetch};

/// Provides read/write access to a component storage. Implements [`Fetch`](../trait.Fetch.html) to
/// provide mutable iterators to [`StorageTuple`s](../storage_tuple/trait.StorageTuple.html)
pub struct Write<'a, C> {
    storage: &'a mut ComponentStorage<C>,
    guard: Option<RwLockWriteGuard<'a, Vec<C>>>,
}

impl<'a, C> Write<'a, C> {
    pub fn new(storage: &'a mut ComponentStorage<C>) -> Write<'a, C> {
        Write { storage, guard: None, }
    }
}

impl<'a, C> Fetch<'a> for Write<'a, C> {
    type Component = &'a mut C;
    type Iterator = IterMut<'a, C>;
    type Guard = RwLockWriteGuard<'a, Vec<C>>;

    fn fetch_accessor(&'a mut self) -> &Self::Guard {
        self.guard = Some(self.storage.fetch_for_writing());
        // FIXME: Returning a ref to the guard might be unnecessary
        self.guard.as_ref().unwrap()
    }
}

impl<'a, 'b: 'a, C: 'a> IntoIterator for &'b mut Write<'a, C> {
    type Item = <Write<'a, C> as Fetch<'a>>::Component;
    type IntoIter = <Write<'a, C> as Fetch<'a>>::Iterator;

    fn into_iter(self) -> Self::IntoIter {
        self.guard.as_mut().unwrap().iter_mut()
    }
}

