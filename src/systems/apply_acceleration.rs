use crate::components::{AccelerationComponent, VelocityComponent};
use crate::iter::IterTuple;
use crate::storage::{Read, Write};
use crate::systems::System;

pub struct ApplyAccelerationSystem;

impl<'a> System<'a> for ApplyAccelerationSystem {
    type InputData = (Write<'a, VelocityComponent>,
                      Read<'a, AccelerationComponent>);

    fn tick(&self, (mut velocities, accelerations): Self::InputData) {
        let vel_iter = velocities.iterate();
        let acc_iter = accelerations.iterate();

        for (vel, acc) in IterTuple::from((vel_iter, acc_iter)) {
            vel.value += acc.value;
        }
    }
}
