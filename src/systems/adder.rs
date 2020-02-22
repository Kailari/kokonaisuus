use crate::components::{AmountComponent, ValueComponent};
use crate::systems::System;
use crate::storage::{Write, Read, FetchGuard};

/// System for incrementing values by their respective increments.
pub struct AdderSystem;

impl<'a> System<'a> for AdderSystem {
    type Data = (Write<'a, ValueComponent>,
                 Read<'a, AmountComponent>);

    fn tick(&self, (values, amounts): Self::Data) {
        let data = (values, amounts);
        let accessors = data.claim();
        let iters = accessors.into_iter();
        for (value, amount) in iters {
            value.value += amount.amount;
        };
    }
}
