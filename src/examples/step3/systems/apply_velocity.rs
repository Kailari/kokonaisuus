use std::slice::{Iter, IterMut};

use crate::examples::step3::components::{PositionComponent, VelocityComponent};
use crate::examples::step3::iter::{IteratorTuple, IterTuple};

// Implement `IteratorTuple` for the component iterators required by this "system"
impl<'a> IteratorTuple for (IterMut<'a, PositionComponent>, Iter<'a, VelocityComponent>) {
    type ItemTuple = (&'a mut PositionComponent,
                      &'a VelocityComponent);

    fn next_all(&mut self) -> Option<Self::ItemTuple> {
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
    // TL;DR:   We don't need the `mut` modifier as we do not mutate them _here_. The mutation
    //          occurs inside `IterTuple`
    let pos_iter = positions.iter_mut();
    let vel_iter = velocities.iter();

    // Due to the `IteratorTuple` implementation above, the type parameter constraint on `TupleIter`
    // is fulfilled for tuple `(pos_iter, vel_iter)`, allowing us to construct a `IterTuple` out of
    // them. The `IterTuple` then implements `Iterator`, allowing it to be used with `for`-loops.
    for (pos, vel) in IterTuple::from((pos_iter, vel_iter)) {
        pos.value += vel.value;
    }
}
