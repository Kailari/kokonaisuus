use crate::examples::step3::components::PositionComponent;

// We have a single iterator, just iterate directly
pub fn print_positions(positions: &Vec<PositionComponent>) {
    for pos in positions.iter() {
        println!("Position: {}", pos)
    }
}
