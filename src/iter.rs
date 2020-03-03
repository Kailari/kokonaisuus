//! Provides traits and iterator implementations necessary for conveniently iterating over tuples
//! of iterators.

// Ok, there is an awful lot of code here for doing relatively simple things. What is going on?
//
// Due to rust compilation check called "orphaning rules", we cannot implement the trait `Iterator`
// directly on standard tuples. Why?
//
// Let's say we have a trait called `SomeTrait` and a struct called `SomeStruct`. Both are defined
// in some other library crate and we have them through a dependency. Now, we try to implement the
// trait _in this crate_ as follows:
//
//      impl SomeTrait for SomeStruct { /* ... */ }
//
// Syntactically, this is fine. However, due to the fact that both of those types are from outside
// of our crate, this implementation is now "orphaned", we have no access to it and compilation
// fails. Bummer.
//
// Let's back off a bit. Normally, if we wanted to use functions and methods from `SomeTrait`, we
// would just `use some_crate::SomeTrait;` and all implementations of that trait would be usable
// after that. This is required for all custom traits.
//
// However, now that we have defined an implementation for a trait outside of the crate that it
// belongs to, it is not sensible to activate our implementation by importing the trait from the
// crate the trait originates from. (E.g. This could lead to hard to resolve conflicts if multiple
// crates implemented that trait) Thus, we have no way of reliably bringing our implementation into
// scope in other modules, as we simply have nothing we can import to do that.
//
// If that did not make any sense, this topic is briefly covered in chapter 10.2
//
// (Fun fact/Implementation detail, out of scope for this part)
//  We could use something called newtype-pattern to create a separate wrapper for each tuple
//  we wanted to use and implement iterator on that. Compiler can optimize the "extra layer" away
//  so there would be no runtime costs, however that would require us to create a new struct for
//  each system data tuple, and that could get messy as structs cannot have the same name.
//  Newtype pattern is briefly discussed in chapter 19. ("Advanced traits")


// So, what we actually end up doing to overcome the limitation of not being able to implement
// external traits on external types: we define our own trait.
//
// `IteratorTuple` here is basically just the same as a regular `Iterator`, but the key difference
// is that it is _not_ defined externally and thus we are allowed to implement it on external types.

/// Implement this on the tuple of iterators you want to iterate on.
pub trait IteratorTuple {
    type ItemTuple;

    fn next_all(&mut self) -> Option<Self::ItemTuple>;
}

// Here we define a custom iterator struct. This is a simple wrapper around tuples that implement
// the `IteratorTuple` trait (which allows us to call `next_all` to generate a new item from the
// iterators stored in that tuple)
//
// This then works as follows: In the system, we implement the `IteratorTuple` on the specific tuple
// of iterators that system needs. We are then allowed to wrap that tuple of iterators into
// an `IterTuple` using
//
//      IterTuple::from(tuple_of_iterators)
//
// And then, we implement `Iterator` on `IterTuple` and BOOM, we can use the above snippet in for
// loops. Wrapping it all up:
//
//      impl IteratorTuple for (TypeOfIterA, TypeOfIterB) {
//          // ...
//      }
//
//      // ...
//
//      for (value_a, value_b) in IterTuple::from((iter_a, iter_b)) {
//          // ...
//      }
//
// There is inconvenient amount of boilerplate involved and syntax is not yet very nice, but at
// least we got rid of the `while-let` and got an actual iterator now!

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

// Allows using `IterTuple` in for-loops. Additionally allows calling `.map()`, `.filter()`, etc. on
// it.
impl<T> Iterator for IterTuple<T>
    where T: IteratorTuple
{
    type Item = T::ItemTuple;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterators.next_all()
    }
}
