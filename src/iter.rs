//! Provides traits and iterator implementations necessary for conveniently iterating over tuples
//! of iterators.

pub trait IteratorTuple {
    type ItemTuple;

    fn next_all(&mut self) -> Option<Self::ItemTuple>;
}

macro_rules! implement_iterator_tuple {
    ($( ($i:tt, $item_name:ident, $type_name:ident) ),+) => {
        impl<$( $type_name ),+> IteratorTuple for ($( $type_name ),+)
            where $($type_name: Iterator),+
        {
            type ItemTuple = ($($type_name::Item),+);

            fn next_all(&mut self) -> Option<Self::ItemTuple> {
                match ($(self.$i.next()),+) {
                    ($( Some($item_name) ),+) => Some(($( $item_name ),+)),
                    _ => None,
                }
            }
        }
    };
}

implement_iterator_tuple!((0, a, A), (1, b, B));
implement_iterator_tuple!((0, a, A), (1, b, B), (2, c, C));
implement_iterator_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D));
implement_iterator_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E));
implement_iterator_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F));
implement_iterator_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F), (6, g, G));
implement_iterator_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F), (6, g, G), (7, h, H));
implement_iterator_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F), (6, g, G), (7, h, H), (8, i, I));
implement_iterator_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F), (6, g, G), (7, h, H), (8, i, I), (9, j, J));
implement_iterator_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F), (6, g, G), (7, h, H), (8, i, I), (9, j, J), (10, k, K));
implement_iterator_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F), (6, g, G), (7, h, H), (8, i, I), (9, j, J), (10, k, K), (11, l, L));


pub struct IterTuple<T>
    where T: IteratorTuple
{
    iterators: T,
}

impl<T> From<T> for IterTuple<T>
    where T: IteratorTuple
{
    fn from(iter_tuple: T) -> Self {
        IterTuple { iterators: iter_tuple }
    }
}

impl<T> Iterator for IterTuple<T>
    where T: IteratorTuple
{
    type Item = T::ItemTuple;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterators.next_all()
    }
}
