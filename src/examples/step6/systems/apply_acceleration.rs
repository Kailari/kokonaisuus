use crate::examples::step6::components::{AccelerationComponent, VelocityComponent};
use crate::examples::step6::iter::IterTuple;
use crate::examples::step6::systems::System;

// System itself is a dummy struct without any data. More complex systems, however, could take
// required context- or configuration parameters and store them here.
pub struct ApplyAccelerationSystem;

// The actual changes here:
//  -   The function has been renamed to `tick(...)` and changed to implement method from `System`
//  -   We define the input arguments as an associated type `InputData`
impl<'a> System<'a> for ApplyAccelerationSystem {
    // Here, we now have to define the input data for the tick function. Earlier, when we had the
    // function signature
    //
    //      pub fn apply_acceleration(vs: &mut Vec<VelComp>, as: &Vec<AccComp>) { // ...
    //
    // which was actually syntax-sugared version of
    //
    //      pub fn apply_acceleration<'a>(vs: &'a mut Vec<VelComp>, as: &'a Vec<AccComp>) { // ...
    //
    // Lifetimes of those references could be "elided", as they were self-evident from the function
    // signature. That is:
    //
    //      // We declare the lifetime 'a as this function's lifetime
    //      apply_acceleration<'a>(...)
    //      {   // Function's lifetime 'a starts at this point
    //
    //          // do stuff
    //
    //      }   // Function ends, the its lifetime 'a ends
    //
    // Thus it is self-evident that the parameter's lifetimes are at least as long as the function's
    // lifetime. This allows leaving them out, or "eliding the lifetime parameters".
    //
    // However, now we are defining the type outside the scope of any function. Here, we have no
    // context from which we could figure out the lifetime, so we need to provide a lifetime
    // annotation for those references.
    //
    // Type itself is a straightforward tuple of references to component vectors. Again, references
    // are used to borrow the data instead of moving the ownership. Velocity reference is mutable
    // as we are going to modify the contents of that component storage.
    type InputData = (&'a mut Vec<VelocityComponent>,
                      &'a Vec<AccelerationComponent>);

    // Thing to note here is that we inline de-structure the input data. We name the tuple elements
    // as parameters, so that we don't need to do it later. This trick here is powered by our good
    // friend pattern-matching, refer to chapter 18. for more detailed information
    //
    // Apart from that, everything works just as before!
    fn tick(&self, (velocities, accelerations): Self::InputData) {
        let vel_iter = velocities.iter_mut();
        let acc_iter = accelerations.iter();

        for (vel, acc) in IterTuple::from((vel_iter, acc_iter)) {
            vel.value += acc.value;
        }
    }
}
