use crate::examples::step3::components::{FrictionComponent, VelocityComponent};
use crate::examples::step3::iter::{IteratorTuple, IterTuple};
use std::slice::{IterMut, Iter};

// Implement `IteratorTuple` for the component iterators required by this "system"
impl<'a> IteratorTuple for (IterMut<'a, VelocityComponent>, Iter<'a, FrictionComponent>) {
    type ItemTuple = (&'a mut VelocityComponent,
                      &'a FrictionComponent);

    fn next_all(&mut self) -> Option<Self::ItemTuple> {
        match (self.0.next(), self.1.next()) {
            (Some(vel), Some(fri)) => Some((vel, fri)),
            _ => None,
        }
    }
}

pub fn apply_friction(velocities: &mut Vec<VelocityComponent>, frictions: &Vec<FrictionComponent>) {
    let vel_iter = velocities.iter_mut();
    let fri_iter = frictions.iter();

    for (vel, fri) in IterTuple::from((vel_iter, fri_iter)) {
        if vel.value.length_squared() < f64::EPSILON {
            continue;
        }

        let friction = vel.value.normalize().abs() * fri.value;
        let magnitude_x = (vel.value.x.abs() - friction.x).max(0.0);
        let magnitude_y = (vel.value.y.abs() - friction.y).max(0.0);
        vel.value.x = vel.value.x.signum() * magnitude_x;
        vel.value.y = vel.value.y.signum() * magnitude_y;
    }
}