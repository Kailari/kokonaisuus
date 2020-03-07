use crate::component_storage::ComponentStorage;
use crate::components::{AccelerationComponent, FrictionComponent, PositionComponent, VelocityComponent};
use crate::systems::{ApplyAccelerationSystem, ApplyFrictionSystem, ApplyVelocitySystem, PrintPositionsSystem, PrintStateSystem, System};

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

    pub fn dispatch(&self, storage: &mut ComponentStorage) {
        println!("State before tick:");
        self.print_state.tick((
            storage.fetch_ref::<PositionComponent>().as_ref(),
            storage.fetch_ref::<VelocityComponent>().as_ref(),
            storage.fetch_ref::<AccelerationComponent>().as_ref(),
            storage.fetch_ref::<FrictionComponent>().as_ref(),
        ));

        self.apply_acceleration.tick((
            storage.fetch_mut::<VelocityComponent>().as_mut(),
            storage.fetch_ref::<AccelerationComponent>().as_ref(),
        ));

        self.apply_friction.tick((
            storage.fetch_mut::<VelocityComponent>().as_mut(),
            storage.fetch_ref::<FrictionComponent>().as_ref(),
        ));

        self.apply_velocity.tick((
            storage.fetch_mut::<PositionComponent>().as_mut(),
            storage.fetch_ref::<VelocityComponent>().as_ref(),
        ));

        println!("\nPositions after tick:");
        self.print_positions.tick(
            storage.fetch_ref::<PositionComponent>().as_ref()
        );
    }
}
