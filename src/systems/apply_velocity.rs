use crate::components::{PositionComponent, VelocityComponent};
use crate::iter::IterTuple;
use crate::systems::System;

pub struct ApplyVelocitySystem;

impl<'a> System<'a> for ApplyVelocitySystem {
    type InputData = (&'a mut Vec<PositionComponent>,
                      &'a Vec<VelocityComponent>);
    fn tick(&self, (positions, velocities): Self::InputData) {
        let pos_iter = positions.iter_mut();
        let vel_iter = velocities.iter();

        for (pos, vel) in IterTuple::from((pos_iter, vel_iter)) {
            pos.value += vel.value;
        }
    }
}
