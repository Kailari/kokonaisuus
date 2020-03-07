use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};

use crate::components::{AccelerationComponent, FrictionComponent, PositionComponent, VelocityComponent};

pub struct ComponentStorage {
    positions: RefCell<Vec<PositionComponent>>,
    velocities: RefCell<Vec<VelocityComponent>>,
    accelerations: RefCell<Vec<AccelerationComponent>>,
    frictions: RefCell<Vec<FrictionComponent>>,
}

impl ComponentStorage {
    pub fn new() -> ComponentStorage {
        ComponentStorage {
            positions: RefCell::new(vec![
                PositionComponent::new(0.0, 0.0),
                PositionComponent::new(-42.0, -42.0),
                PositionComponent::new(234.0, 123.0),
                PositionComponent::new(6.0, 9.0),
            ]),
            velocities: RefCell::new(vec![
                VelocityComponent::new(40.0, 10.0),
                VelocityComponent::new(30.0, 20.0),
                VelocityComponent::new(20.0, 30.0),
                VelocityComponent::new(10.0, 40.0),
            ]),
            frictions: RefCell::new(vec![
                FrictionComponent::new(1.0),
                FrictionComponent::new(2.0),
                FrictionComponent::new(3.0),
                FrictionComponent::new(4.0),
            ]),
            accelerations: RefCell::new(vec![
                AccelerationComponent::new(2.0, 16.0),
                AccelerationComponent::new(4.0, 2.0),
                AccelerationComponent::new(8.0, 4.0),
                AccelerationComponent::new(16.0, 8.0),
            ]),
        }
    }

    pub fn fetch_mut<C>(&self) -> RefMut<Vec<C>>
        where C: Any
    {
        self.fetch_component_storage::<C>()
            .borrow_mut()
    }

    pub fn fetch_ref<C>(&self) -> Ref<Vec<C>>
        where C: Any
    {
        self.fetch_component_storage::<C>()
            .borrow()
    }

    fn fetch_component_storage<C>(&self) -> &RefCell<Vec<C>>
        where C: Any
    {
        let component_type_id = TypeId::of::<C>();
        let storage =
            if component_type_id == TypeId::of::<PositionComponent>() {
                &self.positions as &dyn Any
            } else if component_type_id == TypeId::of::<VelocityComponent>() {
                &self.velocities as &dyn Any
            } else if component_type_id == TypeId::of::<AccelerationComponent>() {
                &self.accelerations as &dyn Any
            } else if component_type_id == TypeId::of::<FrictionComponent>() {
                &self.frictions as &dyn Any
            } else {
                panic!("Unknown component type!")
            };

        storage.downcast_ref::<RefCell<Vec<C>>>()
               .expect("Downcasting storage RefCell failed!")
    }
}
