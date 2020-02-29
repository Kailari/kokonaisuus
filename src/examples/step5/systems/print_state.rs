use crate::examples::step5::components::{AccelerationComponent, FrictionComponent, PositionComponent, VelocityComponent};
use crate::examples::step5::iter::IterTuple;

pub fn print_state(
    positions: &Vec<PositionComponent>,
    velocities: &Vec<VelocityComponent>,
    accelerations: &Vec<AccelerationComponent>,
    frictions: &Vec<FrictionComponent>,
) {
    let pos_iter = positions.iter();
    let vel_iter = velocities.iter();
    let acc_iter = accelerations.iter();
    let fri_iter = frictions.iter();

    let mut entity_index = 0;
    for (pos, vel, acc, fri) in IterTuple::from((pos_iter, vel_iter, acc_iter, fri_iter)) {
        println!("Entity{{i={}, Pos[{}], Vel[{}], Acc[{}], Fri[{}]}}", entity_index, pos.value, vel.value, acc.value, fri.value);
        entity_index += 1;
    }
}
