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
