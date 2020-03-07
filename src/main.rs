#![feature(assoc_int_consts)] // Allows use of things like `f64::EPSILON`

use crate::dispatcher::Dispatcher;
use crate::component_storage::ComponentStorage;

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
