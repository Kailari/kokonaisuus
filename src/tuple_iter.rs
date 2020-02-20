use std::slice::IterMut;

use crate::components::{AmountComponent, ValueComponent};

pub trait IterableTuple<'a, A> {
    fn iterator(&'a mut self) -> A;
}

// Compiler bugs out when indexing tuples in macros. Wrapping tuple index to this allows performing
// the indexing operation without getting unexpected token errors.
macro_rules! token_workaround {
    ($x: expr) => ($x);
}

macro_rules! define_iterator_tuple {
    ($tuple_name:ident, $( ($i:tt, $item_name:ident, $types:ty) ),*) => {
        pub struct $tuple_name<'a>($(IterMut<'a, $types>),*);

        impl<'a> Iterator for $tuple_name<'a> {
            type Item = ($(&'a mut $types),*);

            fn next(&mut self) -> Option<Self::Item> {
                #[allow(unused_parens)] // "1-tuples" generates warnings. Suppress them.
                match ($( self.$i.next() ),*) {
                    ($( Some($item_name) ),*) => Some(($( $item_name ),*)),
                    _ => None,
                }
            }
        }

        impl<'a> IterableTuple<'a, $tuple_name<'a>> for (($(&'a mut Vec<$types>),*,)) {
            fn iterator(&'a mut self) -> $tuple_name<'a> {
                $tuple_name ($( token_workaround!(self.$i).iter_mut() ),*)
            }
        }
    };
}

define_iterator_tuple!(ValueIterator, (0, value, ValueComponent));
define_iterator_tuple!(IteratorPairA, (0, value, ValueComponent), (1, amount, AmountComponent));
