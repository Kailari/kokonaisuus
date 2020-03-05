use crate::components::{AccelerationComponent, VelocityComponent};

pub fn apply_acceleration(velocities: &mut Vec<VelocityComponent>, accelerations: &Vec<AccelerationComponent>) {
    let mut vel_iter = velocities.iter_mut();
    let mut acc_iter = accelerations.iter();

    while let (Some(vel), Some(acc)) = (vel_iter.next(), acc_iter.next()) {
        vel.value += acc.value;
    }
}
