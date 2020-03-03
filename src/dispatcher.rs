use crate::systems::{ApplyAccelerationSystem, ApplyFrictionSystem, ApplyVelocitySystem, PrintPositionsSystem, PrintStateSystem, System};
use crate::component_storage::ComponentStorage;

pub struct Dispatcher {
    print_state: PrintStateSystem,
    print_positions: PrintPositionsSystem,
    apply_acceleration: ApplyAccelerationSystem,
    apply_friction: ApplyFrictionSystem,
    apply_velocity: ApplyVelocitySystem,
}

impl Dispatcher {
    pub fn new() -> Dispatcher {
        Dispatcher {
            print_state: PrintStateSystem,
            print_positions: PrintPositionsSystem,
            apply_acceleration: ApplyAccelerationSystem,
            apply_friction: ApplyFrictionSystem,
            apply_velocity: ApplyVelocitySystem,
        }
    }

    // This is almost directly copy-pasted from `main.rs`. Basically the only changes are that the
    // functions we imported previously are now wrapped into a "system struct" and implemented as
    // `System::tick`. Additionally, the component storage vectors are wrapped in another struct
    // called `ComponentStorage`
    pub fn dispatch(&self, components: &mut ComponentStorage) {
        println!("State before tick:");
        self.print_state.tick((&components.positions, &components.velocities, &components.accelerations, &components.frictions));

        self.apply_acceleration.tick((&mut components.velocities, &components.accelerations));
        self.apply_friction.tick((&mut components.velocities, &components.frictions));
        self.apply_velocity.tick((&mut components.positions, &components.velocities));

        println!("\nPositions after tick:");
        self.print_positions.tick(&components.positions);
    }
}
