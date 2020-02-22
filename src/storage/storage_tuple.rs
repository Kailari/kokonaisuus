use std::marker::PhantomData;

use crate::storage::Fetch;

pub struct AccessorTuple<'a, A> {
    _accessors: PhantomData<&'a A>,
}

pub trait FetchGuard<'a>: Sized {
    fn claim(&self) -> AccessorTuple<'a, Self>;
}

impl<'a, A, B> FetchGuard<'a> for (A, B)
    where A: Fetch<'a>,
          B: Fetch<'a> {
    fn claim(&self) -> AccessorTuple<'a, Self> {
        unimplemented!()
    }
}

impl<'a, A, B> IntoIterator for AccessorTuple<'a, (A, B)>
    where A: Fetch<'a>,
          B: Fetch<'a> {
    type Item = (A::Component, B::Component);
    type IntoIter = IterAccessor<'a, (A::Iterator, B::Iterator), Self::Item, (A, B)>;

    fn into_iter(self) -> Self::IntoIter {
        unimplemented!()
    }
}

pub struct IterAccessor<'a, I, V, A> {
    _accessors: PhantomData<&'a A>,
    _iterators: PhantomData<I>,
    _values: PhantomData<V>,
}

impl<'a, A, B> Iterator for IterAccessor<'a, (A::Iterator, B::Iterator), (A::Component, B::Component), (A, B)>
    where A: Fetch<'a>,
          B: Fetch<'a>
{
    type Item = (A::Component, B::Component);

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

// Define traits for tuples with up to 12 elements
//define_tuple!((0, a, A));
//define_tuple!((0, a, A), (1, b, B));
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
