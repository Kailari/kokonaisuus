use crate::components::{AccelerationComponent, FrictionComponent, PositionComponent, VelocityComponent};
use crate::iter::IterTuple;
use crate::storage::Read;
use crate::systems::System;

pub struct PrintStateSystem;

impl<'a> System<'a> for PrintStateSystem {
    type InputData = (Read<'a, PositionComponent>,
                      Read<'a, VelocityComponent>,
                      Read<'a, AccelerationComponent>,
                      Read<'a, FrictionComponent>);

    fn tick(&self, (positions, velocities, accelerations, frictions): Self::InputData) {
        let pos_iter = positions.iterate();
        let vel_iter = velocities.iterate();
        let acc_iter = accelerations.iterate();
        let fri_iter = frictions.iterate();

        for (entity_index, (pos, vel, acc, fri)) in IterTuple::from((pos_iter, vel_iter, acc_iter, fri_iter)).enumerate() {
            println!("Entity{{i={}, Pos[{}], Vel[{}], Acc[{}], Fri[{}]}}",
                     entity_index,
                     pos.value,
                     vel.value,
                     acc.value,
                     fri.value);
        }
    }
}
