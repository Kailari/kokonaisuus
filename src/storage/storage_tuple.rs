use crate::storage::StorageLock;

/// Iterator over tuples which implement `IteratorTuple`. The actual implementations are generated
/// using macros.
pub struct TupleIter<T: IteratorTuple> {
    iterators: T,
}

/// Trait for tuples containing iterators. Provides method for creating item containing all items
/// from wrapped iterators.
pub trait IteratorTuple: Sized {
    type Item;

    fn next_all(&mut self) -> Option<Self::Item>;
}

/// Allows converting a tuple of iterators into a `TupleIter`
pub trait IntoIteratorTuple: Sized {
    type Iterators: IteratorTuple;

    fn iterator(self) -> TupleIter<Self::Iterators>;
}

/// Iterator implementation for `TupleIter`. Exploits the fact that all supported tuples implement
/// the `IteratorTuple`-trait, which makes producing items trivial.
impl<T: IteratorTuple> Iterator for TupleIter<T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterators.next_all()
    }
}

macro_rules! define_tuple {
    // Implements all three traits for tuples with given length (`StorageLock`, `IteratorTuple` and
    // `IntoIteratorTuple`) using the three macro variants
    ($(($i:tt, $item_name:ident, $type_name:ident)),+) => {
        define_tuple! { stor $(($i, $type_name)),+ }
        define_tuple! { into $(($i, $type_name)),+ }
        define_tuple! { iter $(($i, $item_name, $type_name)),+ }
    };

    (stor $(($i:tt, $type_name:ident)),+) => {
        /// Implementation of `StorageLock` for tuples containing only `StorageLocks`. Allows
        /// shortening this...
        /// ```
        /// let components_a = &lock_a.claim();
        /// let components_b = &lock_b.claim();
        /// ```
        /// ...to something like:
        /// ```
        /// let (components_a, components_b) = (&lock_a, &lock_b).claim();
        /// ```
        impl<'a: 'b, 'b, $($type_name),+> StorageLock<'a, 'b> for ($($type_name),+,)
            where $($type_name: StorageLock<'a, 'b>),+
        {
            type Accessor = ($($type_name::Accessor),+,);

            fn claim(self) -> Self::Accessor {
                ($(self.$i.claim()),+,)
            }
        }
    };

    (iter $(($i:tt, $item_name:ident, $type_name:ident)),+) => {
        /// Implementation of `IteratorTuple` for tuples containing only `Iterator`s. Allows
        /// conveniently conveniently iterating over a tuple of iterators as a single iterator which
        /// produces tuples of wrapped iterators' items.
        ///
        /// Returns `None` as soon as any of the iterators returns `None`.
        /// ```
        /// let iter_a = vec_a.iter();
        /// let iter_b = vec_b.iter();
        ///
        /// // Iterating with `TupleIter` becomes possible
        /// let wrapper = TupleIter { iterators: (iter_a, iter_b) };
        ///
        /// for (value_from_vec_a, value_from_vec_b) in wrapper {
        ///     // ...
        /// }
        /// ```
        impl<$($type_name),+> IteratorTuple for ($($type_name),+,)
            where $($type_name: Iterator),+
        {
            type Item = ($($type_name::Item),+,);

            fn next_all(&mut self) -> Option<Self::Item> {
                match ($(self.$i.next()),+,) {
                    ($(Some($item_name)),+,) => Some(($($item_name),+,)),
                    _ => None,
                }
            }
        }
    };

    (into $(($i:tt, $type_name:ident)),+) => {
        /// Implementation of `IntoIteratorTuple` for tuples containing only `IntoIterator`s.
        /// Converts all tuple elements into iterators and creates a `TupleIter` out of them. This
        /// relies on tuples having the `IteratorTuple` implemented on them. Allows us to write:
        /// ```
        /// // This assumes that collection_a and collection_b both implement `IntoIterator`
        /// for (value_from_a, value_from_b) in (collection_a, collection_b).iterator() {
        ///     // ...
        /// }
        /// ```
        impl<'a, $($type_name),+> IntoIteratorTuple for ($($type_name),+,)
            where $($type_name: IntoIterator),+
        {
            type Iterators = ($($type_name::IntoIter),+,);

            fn iterator(self) -> TupleIter<Self::Iterators> {
                TupleIter { iterators: ($(self.$i.into_iter()),+,) }
            }
        }
    }
}

// Define traits for tuples with up to 12 elements. This allows:
//  1. claiming tuples of storage locks with a single call
//  2. converting tuples of iterators into iterators
define_tuple!((0, a, A));
define_tuple!((0, a, A), (1, b, B));
define_tuple!((0, a, A), (1, b, B), (2, c, C));
define_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D));
define_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E));
define_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F));
define_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F), (6, g, G));
define_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F), (6, g, G), (7, h, H));
define_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F), (6, g, G), (7, h, H), (8, i, I));
define_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F), (6, g, G), (7, h, H), (8, i, I), (9, j, J));
define_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F), (6, g, G), (7, h, H), (8, i, I), (9, j, J), (10, k, K));
define_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F), (6, g, G), (7, h, H), (8, i, I), (9, j, J), (10, k, K), (11, l, L));
