use crate::components::PositionComponent;

pub fn print_positions(positions: &Vec<PositionComponent>) {
    let mut pos_iter = positions.iter();

    while let Some(pos) = pos_iter.next() {
        println!("Position: {}", pos)
    }
}
