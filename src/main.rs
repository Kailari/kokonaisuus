#![feature(assoc_int_consts)] // Allows use of things like `f64::EPSILON`

/*
Step 3. Iterating tuples of iterators
Topics: Custom traits, Iterators, implementing traits on types we haven't defined ourselves

New here:
    -   "Systems" now use `for`-loops for iterating over the component iterators
    -   Created utility traits for combining tuples of iterators into a single tuple-producing
        iterator. Iterators in iterators, yay!
    -   Cleaned up component creation with custom constructor-functions (`Component::new`)

Notes:
    Goal here is to get rid of the ugly `while-let` syntax in systems. Idea is simple: if we have
    an tuple filled with iterators, shouldn't we be allowed to iterate on it? To accomplish this,
    a fair bit of trait magic is required (actually no magic involved, the approach is just a bit
    unintuitive due to language limitations).

    All "systems" implement `IteratorTuple`-trait for their component iterators manually. (See top
    of each system module. `apply_velocity.rs` has the most documentation about what's going on, so
    see that and the `iter.rs`)

    Current one-IteratorTuple-impl-per-system implementation is a bit iffy, but we'll look into that
    in next part.

    TL;DR:
        -   See `apply_velocity.rs`
        -   See `iter.rs`
*/

use self::components::{AccelerationComponent, FrictionComponent, PositionComponent, VelocityComponent};
use self::systems::{apply_velocity, print_positions, apply_friction, apply_acceleration};

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

    apply_acceleration(&mut velocities, &accelerations);
    apply_friction(&mut velocities, &frictions);
    apply_velocity(&mut positions, &velocities);

    print_positions(&positions)
}
