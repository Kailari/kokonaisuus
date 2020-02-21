use crate::components::ValueComponent;
use crate::storage::{Read, StorageTuple};
use crate::systems::System;

/// System for printing the value of a `ValueComponent`
pub struct ValuePrintSystem;

impl<'a> System<'a> for ValuePrintSystem {
    type Data = (Read<'a, ValueComponent>,);

    fn tick(&self, (values,): Self::Data) {
        for (value,) in (values,).iterator() {
            println!("Value: {:?}", value.value)
        }
    }
}
