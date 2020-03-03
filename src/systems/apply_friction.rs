use crate::components::{FrictionComponent, VelocityComponent};
use crate::iter::IterTuple;
use crate::systems::System;

// See `apply_acceleration.rs` for info
pub struct ApplyFrictionSystem;

impl<'a> System<'a> for ApplyFrictionSystem {
    type InputData = (&'a mut Vec<VelocityComponent>, 
                      &'a Vec<FrictionComponent>);
    
    fn tick(&self, (velocities, frictions): Self::InputData) {
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