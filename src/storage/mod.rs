mod read;
mod write;
mod storage_tuple;

pub use read::Read;
pub use write::Write;
pub use storage_tuple::{FetchGuard};

use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

// TODO:    access queue, with parallel read and exclusive write. Calling .iterator() on a storage
//          tuple should block until it can claim all necessary locks. This requires some form of
//          *scheduler* on the dispatcher. Either FIFO or something else. Requirement bitmask
//          generation could help, too.
//          -   Arc<mut C> might be a way to go for tracking usage
//          -   Some mechanism for preventing mutable access while immutable is needed
/// Storage for components. Wraps a vector and controls the access to it.
pub struct ComponentStorage<C> {
    components: Arc<RwLock<Vec<C>>>,
}

impl<'a, C> ComponentStorage<C> {
    /// Constructs a new storage out of the given components
    pub fn from(components: Vec<C>) -> ComponentStorage<C>{
        ComponentStorage { components: Arc::new(RwLock::new(components)) }
    }

    /// Fetches the component vector for reading.
    pub fn fetch_for_reading(&'a self) -> RwLockReadGuard<Vec<C>> {
        self.components.read().unwrap()
    }

    /// Fetches the component vector for writing.
    pub fn fetch_for_writing(&'a mut self) -> RwLockWriteGuard<Vec<C>>  {
        self.components.write().unwrap()
    }
}

/// Accessor used to fetch a guard for claiming R or R/W access to a storage.
pub trait Fetch<'a> {
    type Component;
    type Iterator;
    type Guard;

    fn fetch_accessor(&'a mut self) -> &Self::Guard;
}
