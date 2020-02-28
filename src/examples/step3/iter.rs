//! Provides traits and iterator implementations necessary for conveniently iterating over tuples
//! of iterators.

/// Implement this on the tuple of iterators you want to iterate on.
pub trait IteratorTuple {
    type ItemTuple;

    fn next_all(&mut self) -> Option<Self::ItemTuple>;
}

/// The concrete iterator
pub struct IterTuple<T>
    where T: IteratorTuple
{
    iterators: T,
}

// Allows instantiating `IterTuple` by calling
//
//      let iter_abc = IterTuple::from((iter_a, iter_b, iter_c));
//
// assuming that the type of the tuple `(iter_a, iter_b, iter_c)` implements `IteratorTuple`
impl<T> From<T> for IterTuple<T>
    where T: IteratorTuple
{
    fn from(iter_tuple: T) -> Self {
        IterTuple { iterators: iter_tuple }
    }
}

// Allows using `IterTuple` in for-loops
impl<T> Iterator for IterTuple<T>
    where T: IteratorTuple
{
    type Item = T::ItemTuple;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterators.next_all()
    }
}
