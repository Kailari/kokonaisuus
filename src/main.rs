use crate::components::{AccelerationComponent, FrictionComponent, PositionComponent, VelocityComponent};
use crate::dispatcher::Dispatcher;
use crate::storage::ComponentStorage;

mod components;
mod storage;
mod dispatcher;
mod iter;
mod systems;
mod traits;
mod vector;

pub fn main() {
    let mut storage = ComponentStorage::new(4);
    storage.register_component_type::<PositionComponent>();
    storage.register_component_type::<VelocityComponent>();
    storage.register_component_type::<AccelerationComponent>();
    storage.register_component_type::<FrictionComponent>();

    storage.add_to_entity(0, PositionComponent::new(0.0, 0.0));
    storage.add_to_entity(1, PositionComponent::new(-42.0, -42.0));
    storage.add_to_entity(2, PositionComponent::new(234.0, 123.0));
    storage.add_to_entity(3, PositionComponent::new(6.0, 9.0));

    storage.add_to_entity(0, VelocityComponent::new(40.0, 10.0));
    storage.add_to_entity(1, VelocityComponent::new(30.0, 20.0));
    storage.add_to_entity(3, VelocityComponent::new(10.0, 40.0));

    storage.add_to_entity(0, FrictionComponent::new(1.0));
    storage.add_to_entity(1, FrictionComponent::new(2.0));
    storage.add_to_entity(2, FrictionComponent::new(3.0));
    storage.add_to_entity(3, FrictionComponent::new(4.0));

    storage.add_to_entity(0, AccelerationComponent::new(2.0, 16.0));
    storage.add_to_entity(1, AccelerationComponent::new(4.0, 2.0));
    storage.add_to_entity(2, AccelerationComponent::new(8.0, 4.0));
    storage.add_to_entity(3, AccelerationComponent::new(16.0, 8.0));

    let dispatcher = Dispatcher::new();
    println!("Tick #1:");
    dispatcher.dispatch(&mut storage);

    println!("\nTick #2:");
    dispatcher.dispatch(&mut storage);
}
