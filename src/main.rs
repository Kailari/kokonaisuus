#![feature(assoc_int_consts)] // Allows use of things like `f64::EPSILON`

/*
Step 6. Actual systems
Topics: First peek at lifetimes, lifetime elision in method scope

New here:
    -   Systems now use a shared `System`-trait, which provides a `tick()`-method
    -   System arguments are defined using associated type on system trait
    -   There are lifetimes required! :o (sadly, they are required for odd reasons, but anyways)
    -   Cut-pasted the system initialization and `tick` calls to a new struct `Dispatcher`
    -   Cut-pasted the component initialization to a new struct `ComponentStorage`

Notes:
    In order to be able to start planning the dispatcher, first thing is to unify the way we handle
    the actual system dispatch. The solution was to actually turn system functions into concrete
    instances of something that implements a common `System` -trait.

    Now, when all systems are actual `Systems`, we can call a method from that trait (to be precise,
    `System::tick`) to execute the system. Apart from that, nothing has changed. Trait brought in
    a tad bit of complexity to how we can handle systems requiring varying number of component
    vectors of all sorts of types, but there is nothing too fancy going on.

    In addition to that, I wanted to start switching the mindset away from initializing and doing
    everything in `main()`, so systems and components were wrapped into their own structs,
    `Dispatcher` and `ComponentStorage`, respectively. This should allow us to start setting up
    required framework for unifying the system execution.


    Next up, we need to start taking steps towards further refining component storage, so that we
    could automatically determine which component vectors to pass to `.tick()`. I have very vague
    idea on how this can be done, but it might prove challenging.
*/

use crate::dispatcher::Dispatcher;
use crate::component_storage::ComponentStorage;

mod components;
mod component_storage;
mod dispatcher;
mod iter;
mod systems;
mod vector;

pub fn main() {
    // Yay! No more ugly component initialization in here! Far uglier initialization hidden inside
    // `ComponentStorage` instead!
    let mut components = ComponentStorage::new();

    // Yay again! We have a dumb-impl of a "dispatcher" now. (Take a look inside, it's not very
    // sophisticated yet, but we'll get there, eventually!)
    let dispatcher = Dispatcher::new();
    dispatcher.dispatch(&mut components);
}
