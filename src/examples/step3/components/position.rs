use crate::examples::step3::vector::Vector2d;
use std::fmt::{Display, Formatter};

pub struct PositionComponent {
    pub value: Vector2d,
}

impl PositionComponent {
    pub fn new(x: f64, y: f64) -> Self {
        PositionComponent { value: Vector2d::from((x, y)), }
    }
}

// Implement `Display` for vectors. This allows nice formatting when printing out. Also, this
// auto-magically implements the `ToString` trait (`ToString` has blanket implementation for
// everything which implements `Display`, this is mentioned at the end of chapter 10.2)
impl Display for PositionComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pos[value: {}]", self.value)
    }
}
