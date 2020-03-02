use crate::examples::step7::components::{AccelerationComponent, VelocityComponent};
use crate::examples::step7::iter::IterTuple;
use crate::examples::step7::systems::System;

pub struct ApplyAccelerationSystem;

impl<'a> System<'a> for ApplyAccelerationSystem {
    type InputData = (&'a mut Vec<VelocityComponent>,
                      &'a Vec<AccelerationComponent>);

    fn tick(&self, (velocities, accelerations): Self::InputData) {
        let vel_iter = velocities.iter_mut();
        let acc_iter = accelerations.iter();

        for (vel, acc) in IterTuple::from((vel_iter, acc_iter)) {
            vel.value += acc.value;
        }
    }
}
