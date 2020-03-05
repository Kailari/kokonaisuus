use std::slice::{Iter, IterMut};

use crate::components::{PositionComponent, VelocityComponent};
use crate::iter::{IteratorTuple, IterTuple};

impl<'a> IteratorTuple for (IterMut<'a, PositionComponent>, Iter<'a, VelocityComponent>) {
    type ItemTuple = (<IterMut<'a, PositionComponent> as Iterator>::Item,
                      <Iter<'a, VelocityComponent> as Iterator>::Item);

    fn next_all(&mut self) -> Option<Self::ItemTuple> {
        match (self.0.next(), self.1.next()) {
            (Some(pos), Some(vel)) => Some((pos, vel)),
            _ => None,
        }
    }
}

pub fn apply_velocity(positions: &mut Vec<PositionComponent>, velocities: &Vec<VelocityComponent>) {
    let pos_iter = positions.iter_mut();
    let vel_iter = velocities.iter();

    for (pos, vel) in IterTuple::from((pos_iter, vel_iter)) {
        pos.value += vel.value;
    }
}
