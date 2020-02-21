mod read;
mod write;
mod storage_tuple;

pub use read::Read;
pub use write::Write;
pub use storage_tuple::StorageTuple;

// TODO:    access queue, with parallel read and exclusive write. Calling .iterator() on a storage
//          tuple should block until it can claim all necessary locks. This requires some form of
//          *scheduler* on the dispatcher. Either FIFO or something else. Requirement bitmask
//          generation could help, too.
//          -   Arc<mut C> might be a way to go for tracking usage
//          -   Some mechanism for preventing mutable access while immutable is needed
/// Storage for components. Wraps a vector and controls the access to it.
pub struct ComponentStorage<C> {
    components: Vec<C>,
}

impl<'a, C> ComponentStorage<C> {
    /// Constructs a new storage out of the given components
    pub fn from(components: Vec<C>) -> ComponentStorage<C>{
        ComponentStorage { components }
    }

    /// Fetches the component vector for reading.
    pub fn fetch_for_reading(&'a self) -> &'a Vec<C> {
        &self.components
    }

    /// Fetches the component vector for writing.
    pub fn fetch_for_writing(&'a mut self) -> &'a mut Vec<C> {
        &mut self.components
    }
}

/// Used to fetch an iterator for iterating through a storage.
pub trait Fetch<'a> {
    type Item;
    type Iterator: Iterator<Item=Self::Item>;

    fn fetch_iter(&'a mut self) -> Self::Iterator;
}
