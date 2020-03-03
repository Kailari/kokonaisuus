use crate::components::{FrictionComponent, VelocityComponent};
use crate::iter::IterTuple;

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