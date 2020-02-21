/// Simple component, wrapping a `i32` value used to the value of something.
#[derive(Debug)]
pub struct ValueComponent {
    pub value: i32,
}

/// Simple component, wrapping a `i32` value used to indicate how much a value grows per tick.
#[derive(Debug)]
pub struct AmountComponent {
    pub amount: i32,
}

pub trait Storage<'a> {
    type Component;

    fn fetch_for_reading(&'a self) -> &'a Vec<Self::Component>;

    fn fetch_for_writing(&'a mut self) -> &'a mut Vec<Self::Component>;
}

// TODO:    access queue, with parallel read and exclusive write. Calling .iterator() on a storage
//          tuple should block until it can claim all necessary locks. This requires some form of
//          *scheduler* on the dispatcher. Either FIFO or something else. Requirement bitmask
//          generation could help, too.
pub struct ComponentStorage<C> {
    components: Vec<C>,
}

impl<C> ComponentStorage<C> {
    pub fn from(components: Vec<C>) -> ComponentStorage<C>{
        ComponentStorage { components }
    }
}

impl<'a, C> Storage<'a> for ComponentStorage<C> {
    type Component = C;

    fn fetch_for_reading(&'a self) -> &'a Vec<C> {
        &self.components
    }

    fn fetch_for_writing(&'a mut self) -> &'a mut Vec<C> {
        &mut self.components
    }
}
