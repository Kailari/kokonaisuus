use crate::examples::step6::components::{AccelerationComponent, FrictionComponent, PositionComponent, VelocityComponent};

pub struct ComponentStorage {
    pub positions: Vec<PositionComponent>,
    pub velocities: Vec<VelocityComponent>,
    pub accelerations: Vec<AccelerationComponent>,
    pub frictions: Vec<FrictionComponent>,
}

impl ComponentStorage {
    pub fn new() -> ComponentStorage {
        ComponentStorage {
            positions: vec![
                PositionComponent::new(0.0, 0.0),
                PositionComponent::new(-42.0, -42.0),
                PositionComponent::new(234.0, 123.0),
                PositionComponent::new(6.0, 9.0),
            ],
            velocities: vec![
                VelocityComponent::new(40.0, 10.0),
                VelocityComponent::new(30.0, 20.0),
                VelocityComponent::new(20.0, 30.0),
                VelocityComponent::new(10.0, 40.0),
            ],
            frictions: vec![
                FrictionComponent::new(1.0),
                FrictionComponent::new(2.0),
                FrictionComponent::new(3.0),
                FrictionComponent::new(4.0),
            ],
            accelerations: vec![
                AccelerationComponent::new(2.0, 16.0),
                AccelerationComponent::new(4.0, 2.0),
                AccelerationComponent::new(8.0, 4.0),
                AccelerationComponent::new(16.0, 8.0),
            ],
        }
    }
}
