use crate::components::{PositionComponent, VelocityComponent};
use crate::iter::IterTuple;

pub fn apply_velocity(positions: &mut Vec<PositionComponent>, velocities: &Vec<VelocityComponent>) {
    let pos_iter = positions.iter_mut();
    let vel_iter = velocities.iter();

    for (pos, vel) in IterTuple::from((pos_iter, vel_iter)) {
        pos.value += vel.value;
    }
}
