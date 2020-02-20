use std::slice::IterMut;

pub trait IterableTuple<'a, A> {
    fn iterator(&'a mut self) -> A;
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
    ($tuple_name:ident, $( ($i:tt, $item_name:ident, $type_name:ident) ),*) => {
        pub struct $tuple_name<'a, $( $type_name ),*>($( IterMut<'a, $type_name> ),*);

        impl<'a, $( $type_name ),*> Iterator for $tuple_name<'a, $( $type_name ),*> {
            #[allow(unused_parens)] // "1-tuples" generates warnings. Suppress them.
            type Item = ($(&'a mut $type_name),*);

            fn next(&mut self) -> Option<Self::Item> {
                #[allow(unused_parens)] // "1-tuples" generates warnings. Suppress them.
                match ($( self.$i.next() ),*) {
                    ($( Some($item_name) ),*) => Some(($( $item_name ),*)),
                    _ => None,
                }
            }
        }

        impl<'a, $( $type_name ),*> IterableTuple<'a, $tuple_name<'a, $( $type_name ),*>> for ($( &'a mut Vec<$type_name> ),*,) {
            fn iterator(&'a mut self) -> $tuple_name<'a, $( $type_name ),*> {
                $tuple_name ($( self.$i.iter_mut() ),*)
            }
        }
    };
}

define_iterator_tuple!(ValueIterator, (0, a, A));
define_iterator_tuple!(IteratorPairA, (0, a, A), (1, b, B));
