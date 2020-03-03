//! Provides traits and iterator implementations necessary for conveniently iterating over tuples
//! of iterators.

pub trait IteratorTuple {
    type ItemTuple;

    fn next_all(&mut self) -> Option<Self::ItemTuple>;
}

// The `IteratorTuple` implementation moved from individual systems here. There is a fair bit of
// things to chew on.
//
// Ok, let's go; we had a lot of duplicate code in our `IteratorTuple` implementations and we would
// like to extract the common behavior to a single implementation. That is:
//      "I would like to implement `IteratorTuple` on all permutations of `IterMut` or `Iter` on any
//       permutation of components"
//
// At first sight, this is seemingly impossible. The number of permutations quickly explodes out of
// control. Luckily, after pondering on this a bit, we notice that for all of our systems, two
// conditions are fulfilled:
//      1.  the iterator tuples are always 2-tuples
//      2.  all of the iterators in those tuples have a common level of abstraction, namely,
//          the `Iterator`-trait
//
// NOTE: (printer system is not counted in as it can use its single iterator without any trickery)
//
// So, those two observations in mind, our seemingly impossible issue boils down to something that
// suddenly seems much more manageable:
//      "I would like to implement `IteratorTuple` on all 2-tuples of iterators"
//
// Now, let's get technical for a bit. The `impl`-block is allowed to define arbitrary number of
// type parameters, as long as we constraint them by using them in the trait or the target type.
// In other words:
//
//      impl<A, B> SomeTrait<A> for SomeStruct<B>   // ok!
//      impl<A, B> SomeTrait<A, B> for SomeStruct   // ok!
//      impl<A, B> SomeTrait for SomeStruct<A, B>   // ok!
//      impl<A, B> SomeTrait<(A, B)> for SomeStruct // ok!
//      impl<A, B> SomeTrait<A> for SomeStruct      // error! type parameter B is unconstrained!
//      impl<A, B> SomeTrait for SomeStruct<B>      // error! type parameter A is unconstrained!
//      // etc.
//
// Now how would that be useful to us?
//      "I would like to implement `IteratorTuple`..."  =>  impl<...> IteratorTuple for ...
//      "...on all 2-tuples..."                         =>  impl<A, B> IteratorTuple for (A, B)
//      "...of iterators"                               =>      where A: Iterator,
//                                                                    B: Iterator
//
// See, when you can't find the answer, make sure you are asking the right questions!
//
impl<A, B> IteratorTuple for (A, B)
    where A: Iterator,
          B: Iterator
{
    // Here we don't need to use the fully qualified syntax (`<A as Iterator>::Item`), as we already
    // know from the trait bounds that `A` and `B` are iterators.
    type ItemTuple = (A::Item, B::Item);

    fn next_all(&mut self) -> Option<Self::ItemTuple> {
        // `self` is a 2-tuple (A, B), where A and B are iterators, so it's quite natural we are
        // allowed to call `.next()` on them. We've seen this same matcher pattern before so many
        // times I won't go to detail on how it works.
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
