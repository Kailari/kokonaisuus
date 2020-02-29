use crate::examples::step4::components::{PositionComponent, VelocityComponent};
use crate::examples::step4::iter::IterTuple;

// Apart from getting rid of the boilerplate implementation of the `IteratorTuple` trait, nothing
// has changed here.
pub fn apply_velocity(positions: &mut Vec<PositionComponent>, velocities: &Vec<VelocityComponent>) {
    let pos_iter = positions.iter_mut();
    let vel_iter = velocities.iter();


    for (pos, vel) in IterTuple::from((pos_iter, vel_iter)) {
        pos.value += vel.value;
    }
}
