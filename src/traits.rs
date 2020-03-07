/// Trait for specifying that something can possibly be wrapped to another type of value. For
/// example, this is implemented on component `Option` to tell the compiler that it can unwrapped
/// into a component reference.
///
/// Strangely, the standard library does not seem to provide anything like this, and I'm not willing
/// to add dependency on `core_extensions` just for this.
pub trait OptionLike {
    type Item;

    fn unwrap(self) -> Self::Item;

    fn is_some(&self) -> bool;
}

impl<T> OptionLike for Option<T> {
    type Item = T;

    fn unwrap(self) -> Self::Item {
        self.unwrap()
    }

    fn is_some(&self) -> bool {
        self.is_some()
    }
}

impl<'a, T> OptionLike for &'a Option<T> {
    type Item = &'a T;

    fn unwrap(self) -> Self::Item {
        self.as_ref().unwrap()
    }

    fn is_some(&self) -> bool {
        Option::<T>::is_some(self)
    }
}

impl<'a, T> OptionLike for &'a mut Option<T> {
    type Item = &'a mut T;

    fn unwrap(self) -> Self::Item {
        self.as_mut().unwrap()
    }

    fn is_some(&self) -> bool {
        Option::<T>::is_some(self)
    }
}
