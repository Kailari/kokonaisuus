use crate::storage::{ComponentStorage, Fetch};
use std::slice::Iter;

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
