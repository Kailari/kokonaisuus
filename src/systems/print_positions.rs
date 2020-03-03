use crate::components::PositionComponent;
use crate::systems::System;

// See `apply_acceleration.rs` for info
pub struct PrintPositionsSystem;

impl<'a> System<'a> for PrintPositionsSystem {
    type InputData = &'a Vec<PositionComponent>;

    fn tick(&self, positions: Self::InputData) {
        for pos in positions.iter() {
            println!("Position: {}", pos)
        }
    }
}
