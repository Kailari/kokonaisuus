use crate::storage::{ComponentStorage, Fetch};
use std::slice::IterMut;

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