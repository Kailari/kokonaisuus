use crate::examples::step2::components::{VelocityComponent, PositionComponent};

pub fn apply_velocity(positions: &mut Vec<PositionComponent>, velocities: &Vec<VelocityComponent>) {
    let mut pos_iter = positions.iter_mut();
    let mut vel_iter = velocities.iter();

    while let (Some(pos), Some(vel)) = (pos_iter.next(), vel_iter.next()) {
        pos.value += vel.value;
    }
}
