use crate::components::{AmountComponent, ValueComponent};
use crate::storage::{Read, StorageLock, IntoIteratorTuple};
use crate::systems::System;

/// System for printing the value and increment pairs.
pub struct PrintSystem;

impl<'a> System<'a> for PrintSystem {
    type Data = (Read<'a, ValueComponent>,
                 Read<'a, AmountComponent>);

    fn tick(&self, data: Self::Data) {
        let (values, amounts) = data.claim();
        for (value, amount) in (&values, &amounts).iterator() {
            println!("Value: {:?}, Amount: {:?}", value.value, amount.amount)
        }
    }
}
