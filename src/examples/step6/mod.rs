/*
Step 6.
Topics:

New here:
    -   TODO

Notes:

*/

use self::components::{AccelerationComponent, FrictionComponent, PositionComponent, VelocityComponent};
use crate::examples::step6::systems::{PrintStateSystem, ApplyAccelerationSystem, ApplyFrictionSystem, ApplyVelocitySystem, PrintPositionsSystem, System};

mod components;
mod systems;
mod vector;
mod iter;

pub fn main() {
    let mut positions = vec![
        PositionComponent::new(0.0, 0.0),
        PositionComponent::new(-42.0, -42.0),
        PositionComponent::new(234.0, 123.0),
        PositionComponent::new(6.0, 9.0),
    ];
    let mut velocities = vec![
        VelocityComponent::new(40.0, 10.0),
        VelocityComponent::new(30.0, 20.0),
        VelocityComponent::new(20.0, 30.0),
        VelocityComponent::new(10.0, 40.0),
    ];
    let frictions = vec![
        FrictionComponent::new(1.0),
        FrictionComponent::new(2.0),
        FrictionComponent::new(3.0),
        FrictionComponent::new(4.0),
    ];
    let accelerations = vec![
        AccelerationComponent::new(2.0, 16.0),
        AccelerationComponent::new(4.0, 2.0),
        AccelerationComponent::new(8.0, 4.0),
        AccelerationComponent::new(16.0, 8.0),
    ];

    let print_state = PrintStateSystem;
    let print_positions = PrintPositionsSystem;
    let apply_acceleration = ApplyAccelerationSystem;
    let apply_friction = ApplyFrictionSystem;
    let apply_velocity = ApplyVelocitySystem;

    // Print the initial state
    println!("Initial state:");
    print_state.tick((&positions, &velocities, &accelerations, &frictions));

    apply_acceleration.tick((&mut velocities, &accelerations));
    apply_friction.tick((&mut velocities, &frictions));
    apply_velocity.tick((&mut positions, &velocities));

    println!("\nPositions at the end:");
    print_positions.tick((&positions));
}
