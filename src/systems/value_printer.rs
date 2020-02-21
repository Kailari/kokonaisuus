use crate::components::ValueComponent;
use crate::storage::Read;
use crate::storage_tuple_iter::StorageTuple;
use crate::systems::System;

/// System for printing the value of a `ValueComponent`
pub struct ValuePrintSystem;

impl<'a> System<'a> for ValuePrintSystem {
    type Data = (Read<'a, ValueComponent>,
                 Read<'a, ValueComponent>);

    fn tick(&self, (values, values2): Self::Data) {
        for (value, _) in (values, values2).iterator() {
            println!("Value: {:?}", value.value)
        }
    }
}
