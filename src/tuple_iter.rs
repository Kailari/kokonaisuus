use std::marker::PhantomData;
use std::slice::IterMut;

use crate::components::{Storage, ComponentStorage};

/// Tuple of storages. Something we want to be able to create component/data iterator from.
pub trait StorageTuple<'a> {
    type Items;
    type Iterators: IteratorTuple<Item=Self::Items>;

    fn iterator(&'a mut self) -> TupleIter<Self::Iterators, Self::Items>;
}

/// Tuple of iterators. Something we want to iterate on.
pub trait IteratorTuple {
    type Item;

    fn next_item(&mut self) -> Option<Self::Item>;
}

/// Concrete tuple iterator. Uses `IteratorTuple` to iterate on tuples of iterators.
pub struct TupleIter<I, V>
    where I: IteratorTuple<Item=V> {
    iterators: I,
    // "Phantom" is zero-sized data-type (thus does not exist at runtime) used to mark unused type
    // parameters as required when they are not actually used in struct definition, but needed in
    // implementation.
    values_phantom: PhantomData<V>,
}

impl<I, V> TupleIter<I, V>
    where I: IteratorTuple<Item=V> {
    fn from(iterators: I) -> TupleIter<I, V> {
        TupleIter {
            iterators,
            values_phantom: PhantomData,
        }
    }
}

impl<I, V> Iterator for TupleIter<I, V>
    where I: IteratorTuple<Item=V> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterators.next_item()
    }
}

// Compiler bugs out in some specific scenarios when indexing tuples in macros. Wrapping the tuple
// element index with this allows performing the indexing operation without getting errors due to
// unexpected tokens.
// XXX: Not needed on certain toolchain versions? Works on desktop but not on arch laptop. This
//      issue needs more investigation.
// macro_rules! token_workaround {
//     ($x: expr) => ($x);
// }

macro_rules! define_iterator_tuple {
    ($( ($i:tt, $item_name:ident, $type_name:ident) ),*) => {
        // Implement `StorageTuple` for tuple of vectors
        impl<'a, $($type_name),*> StorageTuple<'a> for ($( &'a mut ComponentStorage<$type_name> ),*,) {
            type Iterators = ($( IterMut<'a, $type_name> ),*,);
            type Items = ($( &'a mut $type_name ),*,);

            fn iterator(&'a mut self) -> TupleIter<Self::Iterators, Self::Items> {
                TupleIter::from(($( self.$i.fetch_for_writing().iter_mut() ),*,))
            }
        }

        // Implement `IteratorTuple` for tuple of iterators
        impl<'a, $( $type_name ),*> IteratorTuple for ($( IterMut<'a, $type_name> ),*,) {
            type Item = ($(&'a mut $type_name),*,);

            fn next_item(&mut self) -> Option<Self::Item> {
                match ($( self.$i.next() ),*,) {
                    ($( Some($item_name) ),*,) => Some(($( $item_name ),*,)),
                    _ => None,
                }
            }
        }
    };
}

define_iterator_tuple!((0, a, A));
define_iterator_tuple!((0, a, A), (1, b, B));
define_iterator_tuple!((0, a, A), (1, b, B), (2, c, C));
define_iterator_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D));
define_iterator_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E));
define_iterator_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F));
define_iterator_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F), (6, g, G));
define_iterator_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F), (6, g, G), (7, h, H));
define_iterator_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F), (6, g, G), (7, h, H), (8, i, I));
define_iterator_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F), (6, g, G), (7, h, H), (8, i, I), (9, j, J));
define_iterator_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F), (6, g, G), (7, h, H), (8, i, I), (9, j, J), (10, k, K));
define_iterator_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F), (6, g, G), (7, h, H), (8, i, I), (9, j, J), (10, k, K), (11, l, L));
