pub struct FrictionComponent {
    pub value: f64,
}

impl From<f64> for FrictionComponent {
    fn from(source: f64) -> Self {
        FrictionComponent { value: source }
    }
}
