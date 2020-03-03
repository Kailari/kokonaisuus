mod apply_acceleration;
mod apply_friction;
mod apply_velocity;
mod print_positions;
mod print_state;

pub use self::apply_velocity::ApplyVelocitySystem;
pub use self::apply_friction::ApplyFrictionSystem;
pub use self::apply_acceleration::ApplyAccelerationSystem;
pub use self::print_positions::PrintPositionsSystem;
pub use self::print_state::PrintStateSystem;


// Here we define a common trait for the systems. Each system provides a `tick()` -method which then
// executes whatever data manipulation the system is supposed to perform.
//
// `InputData` is associated type in which we provide to allow the system implementations to specify
// what kind of data they are willing to accept.
//
// The trait defines a "lifetime parameter" as defining the `InputData` commonly requires reference
// types, which in turn require compile-time lifetime annotations.
pub trait System<'a> {
    type InputData;

    fn tick(&self, data: Self::InputData);
}
