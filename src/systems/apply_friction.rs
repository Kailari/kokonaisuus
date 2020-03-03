use crate::components::{FrictionComponent, VelocityComponent};
use crate::iter::{IteratorTuple, IterTuple};
use std::slice::{IterMut, Iter};

// Implement `IteratorTuple` for the component iterators required by this "system"
impl<'a> IteratorTuple for (IterMut<'a, VelocityComponent>, Iter<'a, FrictionComponent>) {
    // Here is an example without fully-qualified trait syntax. When `VelocityComponent` and
    // `FrictionComponent` get passed as type parameters down to `IterMut` and `Iter`, these are
    // the types those iterators will produce (that is `Iterator::Item` for those iterators)
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
        let magnitude = (vel.value.abs() - friction).max(0.0);
        vel.value.x = vel.value.x.signum() * magnitude.x;
        vel.value.y = vel.value.y.signum() * magnitude.y;
    }
}