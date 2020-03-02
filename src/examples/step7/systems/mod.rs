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


pub trait System<'a> {
    type InputData;

    fn tick(&self, data: Self::InputData);
}
