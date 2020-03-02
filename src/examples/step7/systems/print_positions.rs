use crate::examples::step7::components::PositionComponent;
use crate::examples::step7::systems::System;

pub struct PrintPositionsSystem;

impl<'a> System<'a> for PrintPositionsSystem {
    type InputData = &'a Vec<PositionComponent>;

    fn tick(&self, positions: Self::InputData) {
        for pos in positions.iter() {
            println!("Position: {}", pos)
        }
    }
}
