use crate::components::{AccelerationComponent, VelocityComponent};
use crate::iter::IterTuple;

pub fn apply_acceleration(velocities: &mut Vec<VelocityComponent>, accelerations: &Vec<AccelerationComponent>) {
    let vel_iter = velocities.iter_mut();
    let acc_iter = accelerations.iter();

    for (vel, acc) in IterTuple::from((vel_iter, acc_iter)) {
        vel.value += acc.value;
    }
}
