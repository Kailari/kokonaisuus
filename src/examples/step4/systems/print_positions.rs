use crate::examples::step4::components::PositionComponent;

pub fn print_positions(positions: &Vec<PositionComponent>) {
    for pos in positions.iter() {
        println!("Position: {}", pos)
    }
}
