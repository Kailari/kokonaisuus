use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};

use crate::examples::step7::components::{AccelerationComponent, FrictionComponent, PositionComponent, VelocityComponent};

// ...what the heck? There are two new things that are directly visible here, and lots of other new
// stuff this allows us to do. So, let's start with the first obvious one, the `Box`.
//
// We are wrapping component storages to boxes here, this is called "boxing" and might be familiar
// from other languages. In case it isn't, by boxing, we allocate the fields on the heap instead of
// allocating on the stack. From `std::boxed`-module documentation:
//      "A pointer type for heap allocation.
//
//       Box<T>, casually referred to as a 'box', provides the simplest form of heap allocation in
//       Rust. Boxes provide ownership for this allocation, and drop their contents when they go
//       out of scope."
//
// So in essence, the boxes allow us to allocate things on the heap. Nice, now why is this useful?
// Anything stored on the stack must have known size at compile time. Now that we are taking steps
// towards unifying our component storage logic, soon we won't know beforehand (at compile-time)
// what types of components there will be in the vectors we have in the `ComponentStorage`
//
// What? Why? Ok, let's try explaining that again: There is no convenient way of defining some "map"
// data-structure that could house our component storage vectors, knowing their exact sizes from the
// get-go. Such solution *might exist* but writing one would cause the type definitions to explode
// out of control.
//
// Think about how you would store the four component boxes we have here in some data-structure.
// What would happen to the definition of that map when we add an arbitrary number of new component
// types? Writing such implementation is just not feasible.
//
// So, boxes, they allow us to allocate the darn vectors on the heap, so we only need to store a
// pointer here (which has known size at compile time), so presto! All our problems are gone!
//
//
// Well, not all of them. If I said that `dyn Any` is a component storage vector, would you believe
// me? In this case, you have to. TODO

pub struct ComponentStorage {
    positions: Box<dyn Any>,
    velocities: Box<dyn Any>,
    accelerations: Box<dyn Any>,
    frictions: Box<dyn Any>,
}

impl ComponentStorage {
    pub fn new() -> ComponentStorage {
        ComponentStorage {
            positions: Box::new(RefCell::new(vec![
                PositionComponent::new(0.0, 0.0),
                PositionComponent::new(-42.0, -42.0),
                PositionComponent::new(234.0, 123.0),
                PositionComponent::new(6.0, 9.0),
            ])),
            velocities: Box::new(RefCell::new(vec![
                VelocityComponent::new(40.0, 10.0),
                VelocityComponent::new(30.0, 20.0),
                VelocityComponent::new(20.0, 30.0),
                VelocityComponent::new(10.0, 40.0),
            ])),
            frictions: Box::new(RefCell::new(vec![
                FrictionComponent::new(1.0),
                FrictionComponent::new(2.0),
                FrictionComponent::new(3.0),
                FrictionComponent::new(4.0),
            ])),
            accelerations: Box::new(RefCell::new(vec![
                AccelerationComponent::new(2.0, 16.0),
                AccelerationComponent::new(4.0, 2.0),
                AccelerationComponent::new(8.0, 4.0),
                AccelerationComponent::new(16.0, 8.0),
            ])),
        }
    }

    pub fn fetch_mut<C>(&self) -> RefMut<Vec<C>>
        where C: Any
    {
        let component_type_id = TypeId::of::<C>();

        let storage_raw = {
            if component_type_id == TypeId::of::<PositionComponent>() {
                &self.positions
            } else if component_type_id == TypeId::of::<VelocityComponent>() {
                &self.velocities
            } else if component_type_id == TypeId::of::<AccelerationComponent>() {
                &self.accelerations
            } else if component_type_id == TypeId::of::<FrictionComponent>() {
                &self.frictions
            } else {
                panic!("Unknown component type!")
            }
        };

        let storage = storage_raw.downcast_ref::<RefCell<Vec<C>>>()
            .expect("Downcasting mutable storage RefCell failed!");
        storage.borrow_mut()
    }

    pub fn fetch_ref<C>(&self) -> Ref<Vec<C>>
        where C: Any
    {
        let component_type_id = TypeId::of::<C>();

        let storage_raw = {
            if component_type_id == TypeId::of::<PositionComponent>() {
                &self.positions
            } else if component_type_id == TypeId::of::<VelocityComponent>() {
                &self.velocities
            } else if component_type_id == TypeId::of::<AccelerationComponent>() {
                &self.accelerations
            } else if component_type_id == TypeId::of::<FrictionComponent>() {
                &self.frictions
            } else {
                panic!("Unknown component type!")
            }
        };

        let storage = storage_raw.downcast_ref::<RefCell<Vec<C>>>()
            .expect("Downcasting immutable storage RefCell failed!");
        storage.borrow()
    }
}
