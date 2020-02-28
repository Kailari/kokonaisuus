#![feature(assoc_int_consts)] // Allows use of things like `f64::EPSILON`
#![allow(dead_code)] // Examples generate a lot of dead code, allow it

mod systems;
mod components;
mod storage;
mod examples;

#[cfg(feature = "examples")]
fn main () {
    examples::run();
}

#[cfg(not(feature = "examples"))]
fn main() {
    // NOTE: These would be usually at the top of the file. Due to examples feature flag thingies,
    //       these are here to avoid warnings on unused imports when running examples
    use crate::components::{AmountComponent, ValueComponent};
    use crate::systems::{AdderSystem, System, PrintSystem, ValuePrintSystem};
    use crate::storage::{ComponentStorage};

    // Create components
    let mut values = ComponentStorage::from(vec![
        ValueComponent { value: 0 },
        ValueComponent { value: 1 },
        ValueComponent { value: 2 },
        ValueComponent { value: 3 },
    ]);
    let amounts = ComponentStorage::from(vec![
        AmountComponent { amount: 4 },
        AmountComponent { amount: 3 },
        AmountComponent { amount: 2 },
        AmountComponent { amount: 1 },
    ]);

    // Create systems
    let adder = AdderSystem;
    let printer = PrintSystem;
    let value_printer = ValuePrintSystem;

    // Print initial state
    println!("Initial state:");
    printer.tick((values.read(), amounts.read()));

    // Advance a single tick and print state
    adder.tick((values.write(), amounts.read()));
    println!("After tick #1:");
    printer.tick((values.read(), amounts.read()));

    // Advance another tick and print state
    adder.tick((values.write(), amounts.read()));
    println!("After tick #2:");
    printer.tick((values.read(), amounts.read()));

    println!("Values only:");
    value_printer.tick((values.read(),));
}
