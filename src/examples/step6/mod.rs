/*
Step 6. Actual systems
Topics: First peek at lifetimes, lifetime elision in method scope

New here:
    -   Systems now use a shared `System`-trait, which provides a `tick()`-method
    -   System arguments are defined using associated type on system trait
    -   There are lifetimes required! :o (sadly, they are required for odd reasons, but anyways)

Notes:
    In order to be able to start planning the dispatcher, first thing is to unify the way we handle
    the actual system dispatch. The solution was to actually turn system functions into concrete
    instances of something that implements a common `System` -trait.

    Now, when all systems are actual `Systems`, we can call a method from that trait (to be precise,
    `System::tick`) to execute the system. Apart from that, nothing has changed. Trait brought in
    a tad bit of complexity to how we can handle systems requiring varying number of component
    vectors of all sorts of types, but there is nothing too fancy going on.


    Next up, we need to start taking steps towards centralizing component storage, so that we could
    automatically determine which component vectors to pass to `.tick()`. I have very vague idea on
    how this can be done, but it might prove challenging.
*/

use self::components::{AccelerationComponent, FrictionComponent, PositionComponent, VelocityComponent};
use self::systems::{PrintStateSystem, ApplyAccelerationSystem, ApplyFrictionSystem, ApplyVelocitySystem, PrintPositionsSystem, System};

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

    // Note that we are no longer calling "some imported functions from global namespace", but
    // rather `.tick()` method for instances of structs that implement `System`-trait.
    println!("Initial state:");
    print_state.tick((&positions, &velocities, &accelerations, &frictions));

    apply_acceleration.tick((&mut velocities, &accelerations));
    apply_friction.tick((&mut velocities, &frictions));
    apply_velocity.tick((&mut positions, &velocities));

    println!("\nPositions at the end:");
    print_positions.tick(&positions);
}
