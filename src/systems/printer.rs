use crate::components::{AmountComponent, ValueComponent};
use crate::storage::{Read, StorageTuple};
use crate::systems::System;

/// System for printing the value and increment pairs.
pub struct PrintSystem;

impl<'a> System<'a> for PrintSystem {
    type Data = (Read<'a, ValueComponent>,
                 Read<'a, AmountComponent>);

    fn tick(&self, (values, amounts): Self::Data) {
        for (value, amount) in (values, amounts).iterator() {
            println!("Value: {:?}, Amount: {:?}", value.value, amount.amount)
        }
    }
}
