use crate::examples::step2::vector::Vector2d;

pub struct VelocityComponent {
    pub value: Vector2d,
}

impl From<Vector2d> for VelocityComponent {
    fn from(source: Vector2d) -> Self {
        VelocityComponent { value: source, }
    }
}