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

        // And what is going on here?
        //  1.  `IteratorTuple` implements the `Iterator` trait
        //  2.  The implementation states that the `type Item` is tuple of items from the iterators
        //      wrapped (`iter_a` and `iter_b` here)
        //  3.  `next()` -method is implemented to do just that
        //  4.  at top of this file, we have `use crate::tuple_iter::IteratorTuple;` which imports
        //      related `impl Trait` blocks, too, thus bringing the iterator implementation to scope
        //  5.  `for x in y` accepts iterators for the `y` parameter. This basically does
        //          `x = y.next()`
        //      on every iteration.
        //  6.  we de-structure the `x` and get mutable references to the components
        //
        // Thus, after we have wrapped component storage iterators (iter_a and iter_b) to a
        // `TupleIterator`, we can just iterate on the wrapper and it poops out the components as
        // tuples of components.
        for (value, amount) in iter {
            value.value += amount.amount;
        };
    }
}
