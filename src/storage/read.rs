use std::cell::Ref;
use std::slice::Iter;

pub struct Read<'a, C> {
    storage: Ref<'a, Vec<Option<C>>>,
}

impl<'a, C> From<Ref<'a, Vec<Option<C>>>> for Read<'a, C> {
    fn from(source: Ref<'a, Vec<Option<C>>>) -> Self {
        Read { storage: source }
    }
}

impl<'a, C> Read<'a, C> {
    pub fn iterate(&self) -> Iter<Option<C>> {
        self.storage.iter()
    }
}
