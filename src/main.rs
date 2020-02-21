use crate::components::{AmountComponent, ValueComponent};
use crate::systems::{AdderSystem, PrintSystem, System, ValuePrintSystem};
use crate::storage::{ComponentStorage, Write, Read};

mod systems;
mod components;
mod storage;

fn main() {
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
    printer.tick((Read::new(&values), Read::new(&amounts)));

    // Advance a single tick and print state
    println!("After tick #1:");
    adder.tick((Write::new(&mut values), Read::new(&amounts)));
    printer.tick((Read::new(&values), Read::new(&amounts)));

    // Advance another tick and print state
    println!("After tick #2:");
    adder.tick((Write::new(&mut values), Read::new(&amounts)));
    printer.tick((Read::new(&values), Read::new(&amounts)));

    println!("Values only:");
    value_printer.tick((Read::new(&values),));
}
