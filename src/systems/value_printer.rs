use crate::components::ValueComponent;
use crate::storage::{IntoIteratorTuple, Read, StorageLock};
use crate::systems::System;

/// System for printing the value of a `ValueComponent`
pub struct ValuePrintSystem;

impl<'a> System<'a> for ValuePrintSystem {
    type Data = (Read<'a, ValueComponent>, );

    fn tick(&self, data: Self::Data) {
        let (values, ) = data.claim();
        for (value, ) in (&values, ).iterator() {
            println!("Value: {:?}", value.value)
        }
    }
}
