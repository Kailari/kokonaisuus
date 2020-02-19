use crate::components::{AmountComponent, ValueComponent};
use crate::systems::System;
use crate::tuple_iter::IteratorTuple;

/// System for incrementing values by their respective increments.
pub struct AdderSystem;

impl<'a> System<'a> for AdderSystem {
    // This is a bit iffy but then again doesn't matter that much. Here we are stating that the
    // referenced component storage vectors have a lifetime at least as long as this system's
    // lifetime. This is required to assure compiler that the data passed to the `.tick()` is valid.
    // It MIGHT BE possible to add lifetime parameter to the `.tick()` itself, but that could cause
    // complications and then again, under what circumstances would we want to drop component
    // storage vectors before or without dropping systems too? (I can't think of any situation where
    // that would be appropriate)
    type Data = (&'a mut Vec<ValueComponent>,
                 &'a mut Vec<AmountComponent>);

    // The tick function itself. "Iterates over data and performs some reading or mutating."
    fn tick(&self, data: Self::Data) {
        // We have *mutable references* to the storage vectors. Thus, we are allowed to create
        // mutable iterators over them. (`Self::Data` is defined as tuple of mutable references to
        // vectors containing components)
        let iter_a = data.0.iter_mut();
        let iter_b = data.1.iter_mut();

        // Create a wrapper `TupleIterator` out of the component storage vectors' iterators
        let iter = IteratorTuple(iter_a, iter_b);

        // Why are we allowed to do this? Where does the `for_each` come from?
        //  1.  `IteratorTuple` implements the `Iterator` trait
        //  2.  The implementation states that the `type Item` is tuple of items from the iterators
        //      wrapped (`iter_a` and `iter_b` here)
        //  3.  `next()` -method is implemented to do just that
        //  4.  at top of this file, we have `use crate::tuple_iter::IteratorTuple;` which imports
        //      related `impl Trait` blocks, too, thus bringing the iterator implementation to scope
        //
        // Thus, after we have wrapped component storage iterators to a TupleIterator, we can just
        // iterate on the wrapper and it poops out the components as pairs.
        iter.for_each(|(value, amount)| {
            value.value += amount.amount
        });
    }
}
