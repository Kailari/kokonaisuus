use crate::components::{FrictionComponent, VelocityComponent};
use crate::iter::IterTuple;
use crate::storage::{Read, Write};
use crate::systems::System;

pub struct ApplyFrictionSystem;

impl<'a> System<'a> for ApplyFrictionSystem {
    type InputData = (Write<'a, VelocityComponent>,
                      Read<'a, FrictionComponent>);

    fn tick(&self, (mut velocities, frictions): Self::InputData) {
        let vel_iter = velocities.iterate();
        let fri_iter = frictions.iterate();

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
}
