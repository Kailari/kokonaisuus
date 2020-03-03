use crate::components::{AccelerationComponent, VelocityComponent};

// Nothing special here, applying using `Vector`-provided `AddAssign`-trait implementation (allows
// the use of `+=` -operator), but apart from that nothing has changed.
pub fn apply_acceleration(velocities: &mut Vec<VelocityComponent>, accelerations: &Vec<AccelerationComponent>) {
    let mut vel_iter = velocities.iter_mut();
    let mut acc_iter = accelerations.iter();

    while let (Some(vel), Some(acc)) = (vel_iter.next(), acc_iter.next()) {
        vel.value += acc.value;
    }
}
