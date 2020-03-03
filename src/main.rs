#![feature(assoc_int_consts)] // Allows use of things like `f64::EPSILON`

/*
Step 4. Generifying IteratorTuples
Topics: Arbitrary type parameters in trait implementation

New here:
    -   Single implementation for all 2-tuples of iterators. This can be found in `iter.rs`
    -   Other than that, only changes to other files are the fact that the boilerplate-
        implementations of `IteratorTuple` have magically disappeared from system modules.

Notes:
    So, last time we added concept of `IteratorTuple`s using a custom trait. This required us to
    write a lot of boilerplate in system modules. This time, we want to reduce that boilerplate down
    to a single implementation using a few tricks involving generics.

    Note also that our implementation does not care whether or not the specific iterators are
    `IterMut`, `Iter` or some other iterator completely. We can use a single implementation for all
    those cases using the lowest common level of abstraction, the `Iterator`-trait.

    Now, we are limited to 2-tuples as that's the only tuple we've implemented `IteratorTuple` on.
    In next part, we'll look into using macros to generate us a metric ton of other similar
    implementations on larger tuples.
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
