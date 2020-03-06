use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};

use crate::components::{AccelerationComponent, FrictionComponent, PositionComponent, VelocityComponent};

// ...what the heck? There are two new things that are directly visible here, and lots of other new
// stuff this allows us to do. So, let's start with the first obvious one, the `Box`.
//
// We are wrapping component storages to boxes here, this is called "boxing" and might be familiar
// concept from other languages. In case it isn't, by boxing, we allocate the fields on the heap
// instead of allocating on the stack. That means that the allocation occurs on the runtime and
// allocated size does not need to be known at compile-time. From `std::boxed`-module documentation:
//      "`Box` - A pointer type for heap allocation.
//
//       `Box<T>`, casually referred to as a 'box', provides the simplest form of heap allocation
//       in Rust. Boxes provide ownership for this allocation, and drop their contents when they go
//       out of scope."
//
// So in essence, the boxes allow us to allocate things on the heap and we only need to store 
// a pointer (which size is obviously known at compile-time). Nice, now why is this useful?
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
// types? We cannot store the individual vectors in a list-of-lists as they are variable in size!
// Writing such implementation that can store arbitrary number of vectors of arbitrary components
// is just not feasible approach.
//
// So, boxes, they allow us to allocate the darn vectors on the heap, so we only need to store a
// pointer here (again, which has known size at compile time), so presto! All our problems are gone!
//
//
// Well, not *all* of our problems are gone. We just made a plenty more
//
// Now, if I said that the boxed `dyn Any` is a component storage vector, would you believe me?
// If you just did, FYI, I lied, and now the code is broken because you assumed the type wrong.
// So what is going on here actually? (Correct answer: They are `RefCell<Vec<C>>`, not `Vec<C>`)
//
// Compromises. Again. `Any` is a trait and `dyn` keyword just means that we are intentionally
// defining a "trait object". Trait objects are how Rust handles polymorphism at instance level; we
// can declare that some boxed value implements the given traits, without knowing its exact type,
// and then use that information to dynamically dispatch the trait methods without knowing what the
// actual implementation will be.
//
// Say, for example the classic `Animal` example, where we have subtypes `Dog` and `Cat` which both
// have the `Animal`-trait. Both implement `Animal::make_noise` differently. Now, we would like to
// store one of those in a struct called `AnimalNoisemaker`, where only thing we care is that the
// animal making noise is an `Animal`. We can write the implementation using trait objects
//
//      struct AnimalNoisemaker {
//          sound_source: Box<dyn Animal>
//      }
//
// Now, we are storing a "Animal trait object" inside the `sound_source` field. Few points:
//  1.  As the `Animal` is a trait and not a struct, we do not know its size at compile time.
//  2.  We do not know the size? Cannot store on the stack.
//  3.  Cannot store on the stack? Heap allocation is required.
//  4.  Heap allocation? `Box<T>`
//
// Additionally:
//  5.  Defining a trait object? Use `dyn`
//
// The last point has not always been the case. Older compiler versions still accept the code just
// fine without the `dyn`-keyword. I'm not 100% sure why the keyword was added, but syntactically
// it is now required so this is one of those "you just need to do it like this", so let's just
// leave it at that.
//
//
// Let's forget about trait objects for a moment. So, I said earlier that I lied and these are
// actually not boxed storage vectors. What are these then?
//
//      positions: Box<RefCell<Vec<PositionComponent>>>,
//
// Above is the full type we store in the `positions`-field. The type is not known at compile-time,
// but rather that is the *runtime* type of that field; type of the value we will store in there.
// We do not use this full type within the struct definition to allow more flexibility for *casting
// between the storage types* when getting a storage vector dynamically. We'll see later what this
// actually means.
//
// So for now it is enough that you understand that:
//  -   `dyn`-keyword is required when defining a trait object
//  -   Trait object is an instance of any type which implements the required trait(s)
//  -   Value can only be stored on the stack if it's size is known at compile time (this is called
//      the `Sized` trait at language level)
//  -   Trait objects (obviously as their actual type is unknown) are not `Sized` so they must be
//      boxed.
//  -   Boxing a value means storing it on the heap (instead of storing it on the stack). Storing
//      on the heap means more expensive runtime memory allocation, so this should be avoided for
//      any short-lived instances.
//  -   Ultimatum: "These fields are boxed trait objects that implement the `Any` trait"
pub struct ComponentStorage {
    positions: RefCell<Vec<PositionComponent>>,
    velocities: RefCell<Vec<VelocityComponent>>,
    accelerations: RefCell<Vec<AccelerationComponent>>,
    frictions: RefCell<Vec<FrictionComponent>>,
}

impl ComponentStorage {
    pub fn new() -> ComponentStorage {
        // Now, we are allowed to do this as `RefCell` implements `Any` (Basically, `Any` is an
        // unique trait that almost everything implements, so what we are doing here is highly
        // illegal and wrong)
        //
        // These field initializers:
        //  1.  Create a new vector containing the components
        //  2.  Wrap the storage vector in a `RefCell` (explained later)
        //  3.  Box the cell to move it to the heap
        //  4.  As type of `positions` is `Box<dyn Any>`, from compiler perspective, all we know 
        //      about it from now on is that it implements `Any`. Yes, we just lost type-safety.
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

    fn fetch_component_storage<C: 'static>(&self) -> &RefCell<Vec<C>> {
        let component_type_id = TypeId::of::<C>();
        let storage = if component_type_id == TypeId::of::<PositionComponent>() {
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
               .expect("Downcasting mutable storage RefCell failed!")
    }
}
