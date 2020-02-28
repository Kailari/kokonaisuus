/*
Step 2. Modules
Topics: Modules

New here:
    -   Components have been moved under a `components` -module
    -   "System"-functions have been moved under a `systems` -module
    -   maths side of things has been simplified with introduction of `Vector`-struct with lots of
        methods and trait implementations for common operations
    -   `Display`-trait implementation for `PositionComponent` for nicer printing

Notes:
    The `From`-trait seems not to have been very good idea for components, probably going to scrap
    it for the next step.

    Import paths are messy in components due to all steps being in a single crate, but that should
    be OK, I guess.
*/

use self::components::{AccelerationComponent, FrictionComponent, PositionComponent, VelocityComponent};
use self::systems::{apply_velocity, print_positions, apply_friction, apply_acceleration};
use self::vector::Vector2d;

mod components;
mod systems;
mod vector;

pub fn main() {
    // `From` -implementations have changed to use the new `Vector` struct. This is getting ugly,
    // but don't worry about it, we'll figure something out! (Only reason we used `From` in the
    // first place was that I wanted to showcase standard library traits, but this is getting
    // unwieldy for this purpose)
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

    // The "system" -methods are now in their own modules, but they function just the same.
    apply_acceleration(&mut velocities, &accelerations);
    apply_friction(&mut velocities, &frictions);
    apply_velocity(&mut positions, &velocities);

    print_positions(&positions)
}
