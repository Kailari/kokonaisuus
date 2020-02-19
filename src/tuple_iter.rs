use std::slice::IterMut;

// TODO: Macro for generating implementations for n-dimensional tuples

/// Simple struct for storing two iterators. Implements `Iterator` to be able to easily iterate on
/// pairs of values from both.
pub struct IteratorTuple<'a, A, B>(pub IterMut<'a, A>, pub IterMut<'a, B>);

// Implement iterator for `TupleIterator`. This implementation works by advancing both of the stored
// iterators and returning `None` once either of the iterators is finished (Iterator is considered
// "finished" once it returns `None` for the first time)
//
// Type parameters represent the items provided by the child iterators.
impl<'a, A, B> Iterator for IteratorTuple<'a, A, B> {
    // Make this iterator produce tuples of the wrapped iterators' items
    type Item = (&'a mut A, &'a mut B);

    fn next(&mut self) -> Option<Self::Item> {
        // This, is where the magic happens. Create a tuple out of the item references returned by
        // wrapped iterators' `.next()` and start matching on that. Note that `Iterator::<T>::next()`
        // actually returns `Option<T>` instead of plain `T` (like e.g. Java would do)
        match (self.0.next(), self.1.next()) {
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

// Trait for creating iterator tuples. This exists mainly because "you cannot impl traits that are
// defined in other crates for arbitrary types" (which prevents us from implementing `Iterator` for
// sized tuples). Creating a separate trait and using that is the "official" way to go, it seems.
//
// In other words:  We cannot write `impl<X, Y> Iterator for (X, Y)` (implement `Iterator` for tuple) as
//                  the compiler prevents this for 'security reasons' as we haven't ourselves declared
//                  neither of those types. Specifically, the trait must be our own to be able to define
//                  it for non-concrete or arbitrary type. (tuples are considered "arbitrary")
// TL;DR:           We cannot implement iterator for tuples due to language constraints. This is for
//                  the `impl`-block below.
pub trait IterableTuple<'a, A, B> {
    fn iterator(&'a mut self) -> IteratorTuple<'a, A, B>;
}

// Allows us to write `(a, b).iterator()` if `a` and `b` are vectors. Return value is a `IteratorTuple`
impl<'a, A, B> IterableTuple<'a, A, B> for (&'a mut Vec<A>, &'a mut Vec<B>) {
    fn iterator(&'a mut self) -> IteratorTuple<'a, A, B> {
        IteratorTuple::<'a, A, B>(self.0.iter_mut(), self.1.iter_mut())
    }
}
