use crate::vector::Vector2d;
use std::fmt::{Display, Formatter};

pub struct PositionComponent {
    pub value: Vector2d,
}

impl From<Vector2d> for PositionComponent {
    fn from(source: Vector2d) -> Self {
        PositionComponent { value: source, }
    }
}

impl Display for PositionComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pos[value: {}]", self.value)
    }
}
