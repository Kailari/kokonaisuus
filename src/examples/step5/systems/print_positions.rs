use crate::examples::step5::components::PositionComponent;

pub fn print_positions(positions: &Vec<PositionComponent>) {
    for pos in positions.iter() {
        println!("Position: {}", pos)
    }
}
