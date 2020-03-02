/*
Step 7.
Topics: Trait objects (the dyn-keyword), Bypassing compile-time borrow checking, Any-trait,
        Heap allocations (Box)

New here:
    -   TODO

Notes:
    This is a seemingly large step, although we accomplish nothing but manage to add rather unsafe
    looking layer of abstraction over our component storages. What we are trying to accomplish is as
    follows:
        1.  Ultimately, the dispatcher needs to be able to call systems' `tick()`, without actually
            knowing what type of data the system expects to receive.
        2.  We DO NOT want to pass the whole component storage to `System::tick` as that would
            prevent dispatcher from knowing what the systems are doing with the data. This would
            prevent the dispatcher from doing "smart" stuff with scheduling systems based on
            component storage Read/Write accesses (simultaneously at most a single write or multiple
            reads are allowed, thus there is a lot room for the scheduler to optimize with the
            execution order)
        2.  Back to topic, first step to achieving convenient storage access is to unify the way we
            get the components from the storage. What this means in practice, we need to get rid of
            the component vector fields on the `ComponentStorage` struct and figure out how to get
            the component storage vector, just using its type

    The last sentence is the important point here. Reworded a bit:
        "We need to figure out how to get component storage vector from the `ComponentStorage`,
         knowing only type of the component"

    OK! This sounds complex, how on earth can we use *the type of something* to index our storage?
    Luckily, our component types are and will most likely be more-or-less known at compile-time, so
    we can use the `std::any::Any`-trait.

    What does `std::any::Any` do? From module documentation:
        "This module implements the Any trait, which enables dynamic typing of any `'static` type
         through runtime reflection."

    As I said, our component types are known at runtime, and so the lifetime of those types is
    considered `'static`. As per the quote from the documentation, we are then allowed to use the
    `Any` trait on them, which "enables dynamic typing through runtime reflection".

    And what does that mean? We get two invaluable utilities: `TypeId` and `Any::downcast_ref/mut`.
    These two allow us to get a compile-time assigned identifier of a type and make assumptions on
    what the types might be. This starts to sound mighty unsafe, and hell yeah, we really need to be
    careful with these.

    In practice, what we want do with these:
        Premise:
            We have a function with type parameter `C`, we can safely assume that `C` is some valid
            component type.

        1.  Figure out what type we are working with using `TypeId::of::<C>()`
        2.  Fetch the component storage vector housing the components of that type (again, using
            the `TypeId` we got in step 1.)
        3.  Perform a downcast on that component vector to convert it to match the type `C`

    For now, we are NOT going to create a centralized storage for component vectors, but rather use
    a dirty hard-coded if-else-mess with hard-coded fields. This is going to look dirty, but
    requires one layer less indirection, so that we can focus on the things required to make the
    three steps above working, instead of worrying about indexing some storage of storages.


    We are slipping towards the darkness and insanity here. This time, for sake of flexibility, we
    had to make small sacrifices on the compile-time borrow- and type-checking. However, this allows
    us to use arbitrary number and combination of components later on, so this is a necessary evil.
    There might be some ways around this, with more sophisticated type-trickery, which could allow
    us to directly assign indices to our component types using custom trait, possibly bypassing the
    need to use facilities from `Any`, but that still does not solve the issues we need `RefCell`
    for.
*/

use crate::examples::step7::dispatcher::Dispatcher;
use crate::examples::step7::component_storage::ComponentStorage;

mod components;
mod component_storage;
mod dispatcher;
mod iter;
mod systems;
mod vector;

pub fn main() {
    let mut components = ComponentStorage::new();

    let dispatcher = Dispatcher::new();
    dispatcher.dispatch(&mut components);
}
