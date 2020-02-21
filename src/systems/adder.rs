use crate::components::{AmountComponent, ValueComponent};
use crate::systems::System;
use crate::storage::{Write, Read};
use crate::storage_tuple_iter::StorageTuple;

/// System for incrementing values by their respective increments.
pub struct AdderSystem;

impl<'a> System<'a> for AdderSystem {
    type Data = (Write<'a, ValueComponent>,
                 Read<'a, AmountComponent>);

    // The tick function itself. "Iterates over data and performs some reading or mutating."
    fn tick(&self, (values, amounts): Self::Data) {
        for (value, amount) in (values, amounts).iterator() {
            value.value += amount.amount;
        };
    }
}
