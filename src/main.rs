#![feature(assoc_int_consts)] // Allows use of things like `f64::EPSILON`

use crate::components::{AccelerationComponent, FrictionComponent, PositionComponent, VelocityComponent};
use crate::systems::{apply_velocity, print_positions, apply_friction, apply_acceleration};
use crate::vector::Vector2d;

mod components;
mod systems;
mod vector;

pub fn main() {
    let mut positions = vec![
        PositionComponent::from(Vector2d::from((0.0, 0.0))),
        PositionComponent::from(Vector2d::from((-42.0, -42.0))),
        PositionComponent::from(Vector2d::from((234.0, 123.0))),
        PositionComponent::from(Vector2d::from((6.0, 9.0))),
    ];
    let mut velocities = vec![
        VelocityComponent::from(Vector2d::from((40.0, 10.0))),
        VelocityComponent::from(Vector2d::from((30.0, 20.0))),
        VelocityComponent::from(Vector2d::from((20.0, 30.0))),
        VelocityComponent::from(Vector2d::from((10.0, 40.0))),
    ];
    let frictions = vec![
        FrictionComponent::from(1.0),
        FrictionComponent::from(2.0),
        FrictionComponent::from(3.0),
        FrictionComponent::from(4.0),
    ];
    let accelerations = vec![
        AccelerationComponent::from(Vector2d::from((2.0, 16.0))),
        AccelerationComponent::from(Vector2d::from((4.0, 2.0))),
        AccelerationComponent::from(Vector2d::from((8.0, 4.0))),
        AccelerationComponent::from(Vector2d::from((16.0, 8.0))),
    ];

    apply_acceleration(&mut velocities, &accelerations);
    apply_friction(&mut velocities, &frictions);
    apply_velocity(&mut positions, &velocities);

    print_positions(&positions)
}
