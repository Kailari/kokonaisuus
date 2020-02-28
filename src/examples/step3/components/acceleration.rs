use crate::examples::step3::vector::Vector2d;

pub struct AccelerationComponent {
    pub value: Vector2d,
}

impl From<Vector2d> for AccelerationComponent {
    fn from(source: Vector2d) -> Self {
        AccelerationComponent { value: source, }
    }
}
