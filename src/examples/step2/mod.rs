/*
Step 2. Modules
Topics: Modules, impl without traits, Add -trait, Mul -trait, AddAssign -trait,
        (bonus) deriving Copy/Clone, (bonus) Display -trait

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

// First of all, these are analogous to java's `import`-statements. There can be multiple imports
// from same module on a single line.
use self::components::{AccelerationComponent, FrictionComponent, PositionComponent, VelocityComponent};
use self::systems::{apply_velocity, print_positions, apply_friction, apply_acceleration};
use self::vector::Vector2d;

// Here we declare submodules. This must always happen in `main.rs`, `lib.rs` or `mod.rs`.
//
// We have two kinds of modules present:
//  A    src/components/mod.rs
//  B    src/components/some_component.rs
//  B    src/vector.rs
//
// A:   "components"-module has submodules, so it has to be defined using a directory called with
//      name "components" and a `mod.rs` directly inside the folder. Anything defined in that
//      `mod.rs` belongs directly to components module and can be imported using
//
//          use crate::components::AnythingDefinedInModFile
//
//      Import may begin with "crate" or "self" if we want to use crate-root (starting from `src/`)
//      or module-root (starting from current file's location) -relative paths. These are required
//      when referring to things inside our current "crate" (project)
//
// B:   "vector"-module has no submodules, thus it is defined directly in a file named `vector.rs`.
//      if later on we would need submodules, the file would need to be be renamed to `mod.rs` and
//      moved to `vector/mod.rs`
//
//      some_component.rs is also a simple module without submodules. It is contained within
//      components-module, thus it is submodule of "components" and `components/mod.rs` must have
//
//          pub mod some_component;
//
//      in order to make it accessible.
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
