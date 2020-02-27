use std::sync::{Arc, RwLock};

pub use read::{Read, StorageReader};
pub use write::{Write, StorageWriter};
pub use storage_tuple::{IteratorTuple, TupleIter, IntoIteratorTuple};

mod read;
mod write;
mod storage_tuple;

/// Storage for components. Wraps a vector and controls the access to it.
pub struct ComponentStorage<C> {
    // So, what on earth is going on with this type?
    //  0.  `Rc<T>`     -   Reference Counted reference of type `T`. Value is dropped once reference
    //                      count reaches zero. Fast but not thread safe, as there are no safeguards
    //                      preventing multiple threads from accessing the reference counter
    //                      simultaneously.
    //  1.  `Arc<T>`    -   Atomically Reference Counted reference for type `T`. Like `Rc` but
    //                      slower and thread safe (Reference counter manipulation is implemented as
    //                      atomic operations). This allows us to refer to the storage from multiple
    //                      threads simultaneously.
    //  2.  `RwLock<T>` -   Read/Write Lock. Allows multiple simultaneous reads or single writer for
    //                      a single resource of type `T` at a time. We use this to provide systems
    //                      with parallel read and exclusive write access to the storage. When read/
    //                      write request cannot be satisfied immediately, `RwLock` auto-magically
    //                      puts the calling threads to a queue.
    //  3.  `Arc<RwLock<Vec<C>>>`   ->  Vector of type `C`, which can be accessed simultaneously
    //                                  from multiple threads, can only be written or read at a time
    //                                  and allows only single simultaneous writer or multiple
    //                                  parallel readers.
    components: Arc<RwLock<Vec<C>>>,
}

/// Wrapper for component storage vector. Provides convenience functions for constructing storage
/// readers/writers.
impl<'a, C> ComponentStorage<C> {
    /// Constructs a new storage out of the given components
    pub fn from(components: Vec<C>) -> ComponentStorage<C> {
        ComponentStorage { components: Arc::new(RwLock::new(components)) }
    }

    /// Fetches the component vector for reading. Call does NOT claim the lock on the storage.
    pub fn read(&'a self) -> Read<'a, C> {
        Read::new(self)
    }

    /// Fetches the component vector for writing. Call does NOT claim the lock on the storage.
    pub fn write(&'a mut self) -> Write<'a, C> {
        Write::new(self)
    }
}

/// Accessor used to fetch a guard for claiming R or R/W access to a storage.
pub trait StorageLock<'a: 'b, 'b> {
    type Accessor;

    /// Locks the referred storage and moves the lock state to the returned accessor. Once the
    /// accessor is dropped, the lock is freed.
    fn claim(self) -> Self::Accessor;
}
