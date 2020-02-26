use std::sync::{Arc, RwLock};

pub use read::{Read, StorageReader};
pub use write::{Write, StorageWriter};
pub use storage_tuple::IteratorTuple;

mod read;
mod write;
mod storage_tuple;

/// Storage for components. Wraps a vector and controls the access to it.
pub struct ComponentStorage<C> {
    components: Arc<RwLock<Vec<C>>>,
}

impl<'a, C> ComponentStorage<C> {
    /// Constructs a new storage out of the given components
    pub fn from(components: Vec<C>) -> ComponentStorage<C> {
        ComponentStorage { components: Arc::new(RwLock::new(components)) }
    }

    /// Fetches the component vector for reading.
    pub fn fetch_for_reading(&'a self) -> Read<'a, C> {
        Read::new(self)
    }

    /// Fetches the component vector for writing.
    pub fn fetch_for_writing(&'a mut self) -> Write<'a, C> {
        Write::new(self)
    }
}

/// Accessor used to fetch a guard for claiming R or R/W access to a storage.
pub trait StorageLock {
    type Accessor;

    fn claim(self) -> Self::Accessor;
}
