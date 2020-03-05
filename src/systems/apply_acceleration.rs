use crate::components::{AccelerationComponent, VelocityComponent};
use crate::iter::{IteratorTuple, IterTuple};
use std::slice::{IterMut, Iter};

impl<'a> IteratorTuple for (IterMut<'a, VelocityComponent>, Iter<'a, AccelerationComponent>) {
    type ItemTuple = (<IterMut<'a, VelocityComponent> as Iterator>::Item,
                      <Iter<'a, AccelerationComponent> as Iterator>::Item);

    fn next_all(&mut self) -> Option<Self::ItemTuple> {
        match (self.0.next(), self.1.next()) {
            (Some(vel), Some(acc)) => Some((vel, acc)),
            _ => None,
        }
    }
}

pub fn apply_acceleration(velocities: &mut Vec<VelocityComponent>, accelerations: &Vec<AccelerationComponent>) {
    let vel_iter = velocities.iter_mut();
    let acc_iter = accelerations.iter();

    for (vel, acc) in IterTuple::from((vel_iter, acc_iter)) {
        vel.value += acc.value;
    }
}
