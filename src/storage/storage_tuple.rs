use crate::storage::StorageLock;

pub struct TupleIter<T: IteratorTuple> {
    iterators: T,
}

pub trait IteratorTuple: Sized {
    type Item;

    fn next_all(&mut self) -> Option<Self::Item>;
}

pub trait IntoIteratorTuple: Sized {
    type Iterators: IteratorTuple;

    fn iterator(self) -> TupleIter<Self::Iterators>;
}

impl<T: IteratorTuple> Iterator for TupleIter<T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterators.next_all()
    }
}

macro_rules! define_tuple {
    ($(($i:tt, $item_name:ident, $type_name:ident)),+) => {
        define_tuple! { stor $(($i, $type_name)),+ }
        define_tuple! { into $(($i, $type_name)),+ }
        define_tuple! { iter $(($i, $item_name, $type_name)),+ }
    };

    (stor $(($i:tt, $type_name:ident)),+) => {
        impl<'a: 'b, 'b, $($type_name),+> StorageLock<'a, 'b> for ($($type_name),+,)
            where $($type_name: StorageLock<'a, 'b>),+
        {
            type Accessor = ($($type_name::Accessor),+,);

            fn claim(&'b self) -> Self::Accessor {
                ($(self.$i.claim()),+,)
            }
        }
    };

    (iter $(($i:tt, $item_name:ident, $type_name:ident)),+) => {
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

// Define traits for tuples with up to 12 elements
define_tuple!((0, a, A));
define_tuple!((0, a, A), (1, b, B));
//define_tuple!((0, a, A), (1, b, B), (2, c, C));
//define_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D));
//define_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E));
//define_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F));
//define_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F), (6, g, G));
//define_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F), (6, g, G), (7, h, H));
//define_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F), (6, g, G), (7, h, H), (8, i, I));
//define_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F), (6, g, G), (7, h, H), (8, i, I), (9, j, J));
//define_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F), (6, g, G), (7, h, H), (8, i, I), (9, j, J), (10, k, K));
//define_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F), (6, g, G), (7, h, H), (8, i, I), (9, j, J), (10, k, K), (11, l, L));
