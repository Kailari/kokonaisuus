use crate::components::{AmountComponent, ValueComponent};
use crate::systems::System;
use crate::tuple_iter::TupleIterator;

/// System for incrementing values by their respective increments.
pub struct AdderSystem;

impl<'a> System<'a> for AdderSystem {
    type Data = (&'a mut Vec<ValueComponent>,
                 &'a mut Vec<AmountComponent>);

    fn tick(&self, data: Self::Data) {
        let iter_a = data.0.iter_mut();
        let iter_b = data.1.iter_mut();

        // Create a wrapper `TupleIterator` out of the component storage vectors' iterators
        let iter = TupleIterator { iter_a, iter_b };

        // Why are we allowed to do this? Where does the `for_each` come from? TupleIterator
        // implements the Iterator trait, so that the `.next()` method produces tuples of items,
        // generated using the iterators it gets as parameters.
        //
        // Thus, after we have wrapped component storage iterators to a TupleIterator, we can just
        // iterate on the wrapper and it poops out the items (components) as pairs.
        iter.for_each(|(value, amount)| {
            value.value += amount.amount
        });
    }
}
