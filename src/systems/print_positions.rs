use crate::components::PositionComponent;
use crate::iter::IterTuple;
use crate::storage::Read;
use crate::systems::System;

pub struct PrintPositionsSystem;

impl<'a> System<'a> for PrintPositionsSystem {
    type InputData = Read<'a, PositionComponent>;

    fn tick(&self, positions: Self::InputData) {
        let pos_iter = positions.iterate();
        for (pos, ) in IterTuple::from((pos_iter, )) {
            println!("Position: {}", pos)
        }
    }
}
