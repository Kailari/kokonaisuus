use crate::examples::step6::vector::Vector2d;

pub struct VelocityComponent {
    pub value: Vector2d,
}

impl VelocityComponent {
    pub fn new(x: f64, y: f64) -> Self {
        VelocityComponent { value: Vector2d::from((x, y)), }
    }
}
