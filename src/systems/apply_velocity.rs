use crate::components::{PositionComponent, VelocityComponent};
use crate::iter::IterTuple;
use crate::storage::{Read, Write};
use crate::systems::System;

pub struct ApplyVelocitySystem;

impl<'a> System<'a> for ApplyVelocitySystem {
    type InputData = (Write<'a, PositionComponent>,
                      Read<'a, VelocityComponent>);

    fn tick(&self, (mut positions, velocities): Self::InputData) {
        let pos_iter = positions.iterate();
        let vel_iter = velocities.iterate();

        for (pos, vel) in IterTuple::from((pos_iter, vel_iter)) {
            pos.value += vel.value;
        }
    }
}
