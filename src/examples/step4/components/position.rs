use crate::examples::step4::vector::Vector2d;
use std::fmt::{Display, Formatter};

pub struct PositionComponent {
    pub value: Vector2d,
}

impl PositionComponent {
    pub fn new(x: f64, y: f64) -> Self {
        PositionComponent { value: Vector2d::from((x, y)), }
    }
}

impl Display for PositionComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pos[value: {}]", self.value)
    }
}
