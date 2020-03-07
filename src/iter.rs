//! Provides traits and iterator implementations necessary for conveniently iterating over tuples
//! of iterators.

use crate::traits::OptionLike;

/// Extended `Option<T>` for detecting situations where only some of the child iterators return a
/// `Some(value)`.
pub enum IteratorTupleOption<T> {
    /// All iterators in the tuple returned a `Some(Some(value))`
    All(T),
    /// All iterators in the tuple returned a `Some(Option)`, but one or more iterators produced
    /// a `Some(None)`.
    Partial,
    /// All iterators in the tuple returned a `None`
    None,
}

pub trait IteratorTuple {
    type ItemTuple;

    fn next_all(&mut self) -> IteratorTupleOption<Self::ItemTuple>;
}

/// Replaces a repetition sequence contents with some other expression.
macro_rules! replace_expr {
    ($_t:tt $sub:tt) => {$sub};
}

macro_rules! implement_iterator_tuple {
    ($( ($i:tt, $item_name:ident, $type_name:ident) ),+) => {
        impl<$( $type_name, )+> IteratorTuple for ($( $type_name, )+)
            where $( $type_name: Iterator, )+
                  $( $type_name::Item: OptionLike, )+
        {
            type ItemTuple = ($( <$type_name::Item as OptionLike>::Item, )+);

            fn next_all(&mut self) -> IteratorTupleOption<Self::ItemTuple> {
                match ($( self.$i.next(), )+) {
                    ($( Some($item_name), )+) if $( $item_name.is_some() )&&+
                    => IteratorTupleOption::All(($( $item_name.unwrap(), )+)),

                    ($( replace_expr!( ($i) None ), )+) => IteratorTupleOption::None,

                    _ => IteratorTupleOption::Partial,
                }
            }
        }
    };
}

implement_iterator_tuple!((0, a, A));
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
    iterators: T
}

impl<'a, T> From<T> for IterTuple<T>
    where T: IteratorTuple
{
    fn from(iter_tuple: T) -> Self {
        IterTuple { iterators: iter_tuple }
    }
}

impl<'a, T> Iterator for IterTuple<T>
    where T: IteratorTuple
{
    type Item = T::ItemTuple;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iterators.next_all() {
                IteratorTupleOption::All(values) => return Some(values),
                IteratorTupleOption::None => return None,
                IteratorTupleOption::Partial => {}
            }
        }
    }
}
