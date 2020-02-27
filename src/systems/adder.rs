use crate::components::{AmountComponent, ValueComponent};
use crate::storage::{IntoIteratorTuple, Read, StorageLock, Write};
use crate::systems::System;

/// System for incrementing values by their respective increments.
pub struct AdderSystem;

impl<'a> System<'a> for AdderSystem {
    type Data = (Write<'a, ValueComponent>,
                 Read<'a, AmountComponent>);

    fn tick(&self, data: Self::Data) {
        {
            let (mut values, amounts) = data.claim();

            for (value, amount) in (&mut values, &amounts).iterator() {
                value.value += amount.amount;
            }
        }

        {
            // FIXME: Do not claim amounts as we do not use it
            let (mut values, _) = data.claim();

            for (value, ) in (&mut values,).iterator() {
                println!("Value after printing: {}", value.value)
            }
        }
    }
}
