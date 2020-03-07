use std::cell::RefMut;
use std::slice::IterMut;

pub struct Write<'a, C> {
    storage: RefMut<'a, Vec<Option<C>>>,
}

impl<'a, C> From<RefMut<'a, Vec<Option<C>>>> for Write<'a, C> {
    fn from(source: RefMut<'a, Vec<Option<C>>>) -> Self {
        Write { storage: source }
    }
}

impl<'a, C> Write<'a, C> {
    pub fn iterate(&mut self) -> IterMut<Option<C>> {
        self.storage.iter_mut()
    }
}
