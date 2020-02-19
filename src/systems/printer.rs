use crate::components::{AmountComponent, ValueComponent};
use crate::systems::System;
use crate::tuple_iter::IteratorTuple;

/// System for printing the value and increment pairs.
pub struct PrintSystem;

impl<'a> System<'a> for PrintSystem {
    type Data = (&'a mut Vec<ValueComponent>,
                 &'a mut Vec<AmountComponent>);

    fn tick(&self, data: Self::Data) {
        IteratorTuple(
            data.0.iter_mut(),
            data.1.iter_mut(),
        ).for_each(|(value, amount)| {
            println!("Value: {:?}, Amount: {:?}", value.value, amount.amount)
        });
    }
}
