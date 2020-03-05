//! Provides traits and iterator implementations necessary for conveniently iterating over tuples
//! of iterators.

pub trait IteratorTuple {
    type ItemTuple;

    fn next_all(&mut self) -> Option<Self::ItemTuple>;
}

impl<A, B> IteratorTuple for (A, B)
    where A: Iterator,
          B: Iterator
{
    type ItemTuple = (A::Item, B::Item);

    fn next_all(&mut self) -> Option<Self::ItemTuple> {
        match (self.0.next(), self.1.next()) {
            (Some(pos), Some(vel)) => Some((pos, vel)),
            _ => None,
        }
    }
}

pub struct IterTuple<T>
    where T: IteratorTuple
{
    iterators: T,
}

impl<T> From<T> for IterTuple<T>
    where T: IteratorTuple
{
    fn from(iter_tuple: T) -> Self {
        IterTuple { iterators: iter_tuple }
    }
}

impl<T> Iterator for IterTuple<T>
    where T: IteratorTuple
{
    type Item = T::ItemTuple;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterators.next_all()
    }
}
