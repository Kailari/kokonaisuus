mod position;
mod velocity;
mod friction;
mod acceleration;

pub use self::{
    position::PositionComponent,
    velocity::VelocityComponent,
    friction::FrictionComponent,
    acceleration::AccelerationComponent,
};
