use crate::examples::step6::components::{FrictionComponent, VelocityComponent};
use crate::examples::step6::iter::IterTuple;
use crate::examples::step6::systems::System;

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
            let magnitude_x = (vel.value.x.abs() - friction.x).max(0.0);
            let magnitude_y = (vel.value.y.abs() - friction.y).max(0.0);
            vel.value.x = vel.value.x.signum() * magnitude_x;
            vel.value.y = vel.value.y.signum() * magnitude_y;
        }
    }
}