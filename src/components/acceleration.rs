use crate::vector::Vector2d;

pub struct AccelerationComponent {
    pub value: Vector2d,
}

impl AccelerationComponent {
    pub fn new(x: f64, y: f64) -> Self {
        AccelerationComponent { value: Vector2d::from((x, y)) }
    }
}
