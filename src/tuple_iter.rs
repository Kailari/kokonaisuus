use std::slice::IterMut;

/// Simple struct for storing two iterators. Implements `Iterator` to be able to easily iterate on
/// pairs of values from both.
pub struct TupleIterator<A, B>
    where A: Iterator,
          B: Iterator
{
    pub iter_a: A,
    pub iter_b: B,
}

/// Implement iterator for TupleIterator. This implementation works by advancing both of the stored
/// iterators and returning None once either of the iterators is finished (Iterator is "finished"
/// once it returns None for the first time)
///
/// Type parameters represent the items provided by the child iterators.
impl<'a, A, B> Iterator for TupleIterator<IterMut<'a, A>, IterMut<'a, B>> {
    // Make this iterator produce tuples of the wrapped iterators' items
    type Item = (&'a mut A, &'a mut B);

    fn next(&mut self) -> Option<Self::Item> {
        // This, is where the magic happens. Create a tuple out of the item references returned by
        // wrapped iterators' `.next()` and start matching on that. Note that `Iterator::<T>::next()`
        // actually returns `Option<T>` instead of plain `T` (like e.g. Java would do)
        match (self.iter_a.next(), self.iter_b.next()) {
            // This line does a lot of stuff:
            //  - On the left side of the matcher arm:
            //      1. match against the tuple (the outer-most parentheses)
            //      2. match situations where both items are Some(x)
            //      3. in addition to matching case where both exist, we have unwrapped the values!
            //         ("unwrap" = "get out of `Optional<T>` or `Result<T, E>`")
            //  - Right side of the matcher arm:
            //      1. Create a new `Option` variant `Some(x)`...
            //      2. ...the `x` being here a tuple constructed out of the unwrapped items!
            (Some(item_a), Some(item_b)) => Some((item_a, item_b)),
            // In all other cases (either one or both were None), just return None
            _ => None,
        }
    }
}
