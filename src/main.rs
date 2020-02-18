use crate::components::{AmountComponent, ValueComponent};
use crate::systems::{AdderSystem, PrintSystem, System};

mod systems;
mod components;
mod tuple_iter;


fn main() {
    // Create components
    let mut value_components = vec![
        ValueComponent { value: 0 },
        ValueComponent { value: 1 },
        ValueComponent { value: 2 },
        ValueComponent { value: 3 },
    ];
    let mut amount_components = vec![
        AmountComponent { amount: 4 },
        AmountComponent { amount: 3 },
        AmountComponent { amount: 2 },
        AmountComponent { amount: 1 },
    ];

    // Create systems
    let adder = AdderSystem;
    let printer = PrintSystem;

    // Print initial state
    println!("Initial state:");
    printer.tick((&mut value_components, &mut amount_components));

    // Advance a single tick and print state
    println!("After tick #1:");
    adder.tick((&mut value_components, &mut amount_components));
    printer.tick((&mut value_components, &mut amount_components));

    // Advance another tick and print state
    println!("After tick #2:");
    adder.tick((&mut value_components, &mut amount_components));
    printer.tick((&mut value_components, &mut amount_components));
}
