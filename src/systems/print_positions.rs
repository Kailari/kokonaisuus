use crate::components::PositionComponent;

// Here, we print using the `Display` implementation on `PositionComponent` instead of manually
// telling how the component value should be displayed. (we pass the whole component to `println`)
pub fn print_positions(positions: &Vec<PositionComponent>) {
    let mut pos_iter = positions.iter();

    while let Some(pos) = pos_iter.next() {
        println!("Position: {}", pos)
    }
}
