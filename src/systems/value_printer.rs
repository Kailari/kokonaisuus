use crate::components::ValueComponent;
use crate::storage::{IntoIteratorTuple, Read, StorageLock};
use crate::systems::System;

/// System for printing the value of a `ValueComponent`
pub struct ValuePrintSystem;

impl<'a> System<'a> for ValuePrintSystem {
    type Data = (Read<'a, ValueComponent>, );

    fn tick(&self, (value_lock,): Self::Data) {
        let (values, ) = (&value_lock,).claim();
        for (value, ) in (&values, ).iterator() {
            println!("Value: {:?}", value.value)
        }
    }
}
