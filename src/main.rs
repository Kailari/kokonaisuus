use crate::components::{AmountComponent, ComponentStorage, ValueComponent};
use crate::systems::{AdderSystem, PrintSystem, System, ValuePrintSystem};

mod systems;
mod components;
mod tuple_iter;

fn main() {
    // Create components
    let mut values = ComponentStorage::from(vec![
        ValueComponent { value: 0 },
        ValueComponent { value: 1 },
        ValueComponent { value: 2 },
        ValueComponent { value: 3 },
    ]);
    let mut amounts = ComponentStorage::from(vec![
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
    printer.tick((&mut values, &mut amounts));

    // Advance a single tick and print state
    println!("After tick #1:");
    adder.tick((&mut values, &mut amounts));
    printer.tick((&mut values, &mut amounts));

    // Advance another tick and print state
    println!("After tick #2:");
    adder.tick((&mut values, &mut amounts));
    printer.tick((&mut values, &mut amounts));

    println!("Values only:");
    value_printer.tick((&mut values, ));
}
