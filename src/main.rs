use crate::component_storage::ComponentStorage;
use crate::components::{AccelerationComponent, FrictionComponent, PositionComponent, VelocityComponent};
use crate::dispatcher::Dispatcher;

mod components;
mod component_storage;
mod dispatcher;
mod iter;
mod systems;
mod vector;

pub fn main() {
    let mut storage = ComponentStorage::new();
    let positions = vec![
        PositionComponent::new(0.0, 0.0),
        PositionComponent::new(-42.0, -42.0),
        PositionComponent::new(234.0, 123.0),
        PositionComponent::new(6.0, 9.0),
    ];
    let velocities = vec![
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

    storage.register_component_type(positions);
    storage.register_component_type(velocities);
    storage.register_component_type(accelerations);
    storage.register_component_type(frictions);

    let dispatcher = Dispatcher::new();
    dispatcher.dispatch(&mut storage);
}
