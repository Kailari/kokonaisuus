use std::slice::{Iter, IterMut};

use crate::examples::step3::components::{PositionComponent, VelocityComponent};
use crate::examples::step3::iter::{IteratorTuple, IterTuple};

// Implement `IteratorTuple` for the component iterators required by this "system"
impl<'a> IteratorTuple for (IterMut<'a, PositionComponent>, Iter<'a, VelocityComponent>) {
    // This strange syntax here is so-called "fully qualified syntax" for accessing traits of some
    // struct. It basically boils down to
    //      <SomeType as SomeTrait>::AssociatedTypeFromThatTrait
    // In this case
    //      SomeType        => IterMut<...> or Iter<...>
    //      SomeTrait       => Iterator
    //      AssociatedType  => Item     (Iterator::Item)
    //
    // So, what happens here, we declare that
    //      "Associated type 'ItemTuple' is a tuple of whatever the wrapped iterators produce"
    //
    // This is possible because we know beforehand that both `IterMut` and `Iter` are iterators,
    // thus treating them as such is completely valid. The compiler cannot know this implicitly as
    // multiple traits can have associated type with same name (and we might not even know precisely
    // what all traits the iterators actually will have), thus we must explicitly tell the compiler
    // that we mean `Iterator::Item` and not `SomeOtherTrait::Item`.
    //
    // For example what this desugars into or what types the `::Item` actually are, there is
    // a version without the syntactic madness in `apply_friction.rs`. This madness becomes much
    // more useful later when we start to generify things.
    //
    // There are other uses for fully qualified syntax, too, but we won't cover them here. For more
    // information refer to chapter 19.2 ("Advanced traits")
    type ItemTuple = (<IterMut<'a, PositionComponent> as Iterator>::Item,
                      <Iter<'a, VelocityComponent> as Iterator>::Item);

    fn next_all(&mut self) -> Option<Self::ItemTuple> {
        // What happens here? This is basically the same `while-let` loop we had in system functions
        // before, but "desugared" into equivalent `match`-expression. Matches tuples of `Option`s
        // from wrapped iterators, unwraps the options and returns the values as a tuple. Returns
        // `None` if any of the wrapped iterators produce a `None` (all other cases, the matcher arm
        // with `_ => None`)
        //
        // For more info on pattern matching, refer to chapter 18. ("Patterns and matching")
        match (self.0.next(), self.1.next()) {
            (Some(pos), Some(vel)) => Some((pos, vel)),
            _ => None,
        }
    }
}

pub fn apply_velocity(positions: &mut Vec<PositionComponent>, velocities: &Vec<VelocityComponent>) {
    // ...iterators are immutable now? How? Actually, they are immutable just in the context of this
    // function. When we pass these iterators to `IterTuple::from(...)` we actually move the
    // ownership there. When ownership moves, the function receiving the ownership can turn these
    // mutable if they want. `mut`-modifier behaves a bit unintuitively when passing by value and
    // ownership moves with the instance.
    //
    // TL;DR:   We don't need the `mut` modifier as we do not mutate the iterators _here_. Actual
    //          mutation occurs inside `IterTuple` _after_ we move the ownership to it, thus they
    //          are not mutable _here_, but it does not prevent them from becoming mutable elsewhere
    //          as they are not borrowed but passed as value with ownership.
    //
    // Refer to chapter 4. ("Understanding Ownership") for more information
    let pos_iter = positions.iter_mut();
    let vel_iter = velocities.iter();

    // Due to the `IteratorTuple` implementation above, the type parameter constraint on `TupleIter`
    // is fulfilled for tuple `(pos_iter, vel_iter)`, allowing us to construct a `IterTuple` out of
    // them. The `IterTuple` then implements `Iterator`, allowing it to be used with `for`-loops.
    //
    // More precisely, when implementing `From<T>` for `IterTuple` in `iter.rs`, we have specified
    // that `T` must be a `IteratorTuple` (the `where T: IteratorTuple` in impl definition). This
    // is called a "trait bound". To satisfy that bound, the tuple `(pos_iter, vel_iter)` must be
    // a `IteratorTuple` or else the `IterTuple::from` would not accept it. For that purpose, we
    // have implemented that trait for this specific occasion at the top of this file.
    for (pos, vel) in IterTuple::from((pos_iter, vel_iter)) {
        pos.value += vel.value;
    }
}
