use crate::components::{AccelerationComponent, FrictionComponent, PositionComponent, VelocityComponent};
use crate::iter::IterTuple;
use crate::systems::System;

// See `apply_acceleration.rs` for info
pub struct PrintStateSystem;

impl<'a> System<'a> for PrintStateSystem {
    type InputData = (&'a Vec<PositionComponent>,
                      &'a Vec<VelocityComponent>,
                      &'a Vec<AccelerationComponent>,
                      &'a Vec<FrictionComponent>);

    fn tick(&self, (positions, velocities, accelerations, frictions): Self::InputData) {
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
}
