mod read;
mod write;

pub use read::Read;
pub use write::Write;

// TODO:    access queue, with parallel read and exclusive write. Calling .iterator() on a storage
//          tuple should block until it can claim all necessary locks. This requires some form of
//          *scheduler* on the dispatcher. Either FIFO or something else. Requirement bitmask
//          generation could help, too.
pub struct ComponentStorage<C> {
    components: Vec<C>,
}

impl<'a, C> ComponentStorage<C> {
    pub fn from(components: Vec<C>) -> ComponentStorage<C>{
        ComponentStorage { components }
    }

    pub fn fetch_for_reading(&'a self) -> &'a Vec<C> {
        &self.components
    }

    pub fn fetch_for_writing(&'a mut self) -> &'a mut Vec<C> {
        &mut self.components
    }
}

/// Used to fetch an iterator for iterating through a storage
pub trait Fetch<'a> {
    type Item;
    type Iterator: Iterator<Item=Self::Item>;

    fn fetch_iter(&'a mut self) -> Self::Iterator;
}
