use crate::components::{AmountComponent, ValueComponent};
use crate::storage::{Read, StorageLock, Write};
use crate::systems::System;

/// System for incrementing values by their respective increments.
pub struct AdderSystem;

impl<'a> System<'a> for AdderSystem {
    type Data = (Write<'a, ValueComponent>,
                 Read<'a, AmountComponent>);

    fn tick(&self, (values, amounts): Self::Data) {
        let mut value_lock = values.claim();
        let amount_lock = amounts.claim();
        let mut value_iter = value_lock.guard.iter_mut();
        let mut amount_iter = amount_lock.guard.iter();
        loop {
            let next_value = value_iter.next();
            let next_amount = amount_iter.next();
            if let (Some(value), Some(amount)) = (next_value, next_amount) {
                value.value += amount.amount;
            } else {
                break;
            }
        }
        //for (value, amount) in (values, amounts).claim() {
        //    value.value += amount.amount;
        //};
    }
}
