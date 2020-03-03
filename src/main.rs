#![feature(assoc_int_consts)] // Allows use of things like `f64::EPSILON`

/*
Step 5. Avoiding writing tons of boilerplate with macros
Topics: macro_rules!

New here:
    -   New system for printing the initial state. This system requires iterating 4-tuples.
    -   Our generic implementation of the 2-tuple `IteratorTuple` has changed to a variadic
        declarative macro, which is then used to generate implementations for n-tuples with n=2..12

Notes:
    Oh no! We have a system which requires 4-tuple of all components. We would have to write another
    `IteratorTuple` implementation to handle 4-tuples but that would be basically duplicate code.
    Worse, if we add more systems in the future, we could need 3-tuples, 5-tuples, 12-tuples, and it
    gets quite unmanageable quite fast.

    Solution: There is a clear pattern on how the implementations expand as tuples grow in size. Use
    "variadic declarative macros" to generate appropriate implementations of different sizes. This
    means that we have all the possible tuple sizes handled, but we only need to write a single
    "blueprint" to handle all of them.

    The "thing" here is that we took the simplest solution available: writing lots of more-or-less
    trivial complexity boilerplate, but skipped the writing part. Now we still have most of
    the elegance of the simple solution intact, without putting in much additional effort!


    Our systems are still just global functions residing in some modules. We probably should do
    something about that next and start designing our dispatcher. Getting even a simple sequential
    dispatcher going will likely take quite a lot of effort, requiring us to re-think how we provide
    the component vectors to the systems, but we'll see how it goes!
*/

use self::components::{AccelerationComponent, FrictionComponent, PositionComponent, VelocityComponent};
use self::systems::{apply_velocity, print_positions, print_state, apply_friction, apply_acceleration};

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

    // Print the initial state
    println!("Initial state:");
    print_state(&positions, &velocities, &accelerations, &frictions);

    apply_acceleration(&mut velocities, &accelerations);
    apply_friction(&mut velocities, &frictions);
    apply_velocity(&mut positions, &velocities);

    println!("\nPositions at the end:");
    print_positions(&positions)
}
