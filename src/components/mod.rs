mod position;
mod velocity;
mod friction;
mod acceleration;

// Note that the modules above are not `pub`. This means that they cannot be accessed from outside
// of this module. We could add the `pub` modifier and import the components whenever we need them
// with
//
//      use crate::components::position::PositionComponent;
//      use crate::components::velocity::VelocityComponent;
// or
//      use crate::components::{
//          position::PositionComponent,
//          velocity::VelocityComponent,
//      };
//
// But that's quite ugly and lot of writing. Instead, we re-export the component structs from those
// modules using `pub use <module>::<WhatWeWantToExport>`, like this:
pub use self::{
    position::PositionComponent,
    velocity::VelocityComponent,
    friction::FrictionComponent,
    acceleration::AccelerationComponent,
};

// Above is equivalent to:
// pub use self::position::PositionComponent;
// pub use self::velocity::VelocityComponent;
// pub use self::friction::FrictionComponent;
// pub use self::acceleration::AccelerationComponent;


// Now, this module (components) re-exports those components, and whenever we need them, we can
// just write:
//
//      use crate::components::{PositionComponent, VelocityComponent};
//
// which is (subjectively) a lot cleaner than without the re-exports
