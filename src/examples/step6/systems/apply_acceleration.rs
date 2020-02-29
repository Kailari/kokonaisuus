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
    //          // do stuff, anything with lifetime 'a is guaranteed to be alive
    //
    //      }   // Function ends, the its lifetime 'a ends
    //
    // Thus it is self-evident that the parameter's lifetimes are at least as long as the function's
    // lifetime. This allows leaving the lifetime annotations out, or in more specific terms, this
    // allows "eliding of the lifetime annotations".
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

    // Now, one might wonder:
    //      "ok, 'a is a lifetime, but what does it represent? Lifetime of _what_, exactly?"
    //
    // Now, that right there, is an exquisite question!
    // Short answer:    lifetime of the reference, whatever it means in that context
    // Long answer:     feeling adventurous? Read along!
    //
    // OUT OF SCOPE ALERT
    // (nothing more in this file, if you are not feeling adventurous, move along to another file)
    //
    // This particular situation is a very odd edge-case of lifetimes, where we are required to
    // provide lifetimes, but there is no convenient way of telling exactly what lifetime we are
    // after, in a meaningful way, without losing generic nature of our implementation.
    //
    // That is, with the old implementation, the lifetimes were *self-evident from the context*.
    // Has our situation changed in any way? No. The lifetimes, *should* still be self-evident in
    // the context of a call to `.tick()`, but they are not! Why?
    //
    // Because we are forced to use an associated type for ensuring that system with any kind of
    // input data can be called with the same `System::tick(&self, data)` method. More precisely,
    // the associated type does not know about the lifetime of any particular call to the tick-
    // method, thus it needs to have "just some arbitrary lifetime", which we seemingly just make
    // out of thin air.
    //
    // Now, in context of a call to `tick()`, the 'a lifetime actually represents just what it did
    // earlier, the lifetime of the method. We just need to have the extra lifetime parameter on
    // the trait for seemingly no reason at all, in order to be able to define the associated type
    // `InputData` properly.
    //
    // EVEN MORE OUT OF SCOPE ALERT
    //
    // In distant future, what we could do is to utilize a upcoming language feature called
    // "generic associated types" (or GAT in short). They allow defining generic type parameters on
    // associated types, so we could then write something like:
    //
    //      impl System for ApplyAccelerationSystem {
    //          type InputData<'a> = (&'a mut Vec<VelComp>, &'a Vec<AccComp>)
    //
    //          fn tick<'a>(&self, (vs, ac): Self::InputData<'a>) {
    //              // ...
    //          }
    //      }
    //
    // which much more accurately conveys what the lifetime on `InputData` means. No need for
    // confusing lifetime parameter on `System`-trait so the implementation is relatively cleaner.
    // (It might even be possible for the compiler to elide the lifetime completely on the function
    //  definition again, as parameter lifetime is in this context self-evident!)
}
