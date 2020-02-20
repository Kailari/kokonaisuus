mod adder;
mod printer;
mod value_printer;

pub use adder::AdderSystem;
pub use printer::PrintSystem;

/// System trait. Provides `tick`-method for running the system.
pub trait System<'a> {
    type Data;

    fn tick(&self, data: Self::Data);
}
