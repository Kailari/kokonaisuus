use crate::components::{ValueComponent, ComponentStorage};
use crate::systems::System;
use crate::tuple_iter::StorageTuple;

/// System for printing the value of a `ValueComponent`
pub struct ValuePrintSystem;

impl<'a> System<'a> for ValuePrintSystem {
    type Data = (&'a mut ComponentStorage<ValueComponent>,);

    fn tick(&self, (values,): Self::Data) {
        for (value,) in (values,).iterator() {
            println!("Value: {:?}", value.value)
        }
    }
}
