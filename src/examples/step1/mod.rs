/*
Step 1. More complex situation
Topics: Implementing traits, `From`-trait, functions, Basic borrowing,
        (bonus) Blanket implementations for traits using other traits

What's new:
    -   more component structs
    -   `From`-trait implementations
    -   while-let loops are now in separate functions. These are the current version of "systems"
        in our ECS

Notes:
    Same thing that we had in the step 0, but more functionality added. Now, in addition to applying
    the velocity, we are first modifying the velocity by acceleration and applying friction to it.
    For purposes of this example, we are applying these (velocity) modifications in separate loops.

    To clean up the main method a bit and to clarify things, the iteration loops are in their own
    methods.

    Additionally, we are implementing the `From`-trait for components for a bit nicer syntax when
    creating the components. These implementations allow converting from tuples to components using
    syntax like `Component::from((some, values))`.

    This file is getting cluttered, next up: splitting to modules and implementing more traits
*/

struct PositionComponent {
    x: f64,
    y: f64,
}

// Here we implement the trait `From<T>` for the position component. `From` is relatively simple
// trait (as traits should be), providing only a single method `fn from(_: T) -> Self`, which simply
// constructs the type we implement the trait on from something with type `T`. For example, here we
// implement `From` for `PositionComponent`, allowing us to create `PositionComponents` using
//
//      let pos = PositionComponent::from(something_with_type_t);
//
// Ok, but what is `T`? The trait `From<T>` has a type parameter `T`, which we can use to tell
// "what type should be convertible to PositionComponent". Here, we define `T` as `(f64, f64)`
// (Tuple of two double-precision floating-point numbers). So, in other words this line reads:
//
//      "Implement trait `From<T>` for PositionComponent, with type parameter T being (f64, f64)"
//
// The trait `From` then uses the `T` as part of its `from`-function's signature. Other thing to
// note here is the use of the `Self`-type, which is just a special type token, allowing traits to
// refer to "the type this trait is being implemented on". In this case, `Self == PositionComponent`
// as we implement the trait on `PositionComponent`
//
// The implementation itself is quite straightforward. As the only parameter to the associated
// function `from` is of type `T`, we get here a tuple of with `f64`s (as we defined T = (f64, f64)
// on the `impl`-line). We can then use this tuple as the `x` and the `y` for the
// `PositionComponent` being constructed. Now, we can:
//
//      let pos = PositionComponent::from((42.0, 6.9));
//
impl From<(f64, f64)> for PositionComponent {
    fn from(source: (f64, f64)) -> Self {
        PositionComponent { x: source.0, y: source.1 }
    }
}


struct VelocityComponent {
    x: f64,
    y: f64,
}

// Fun fact (this thing is out of scope for this step's topic):
//  Traits can have "blanket implementations" on other traits. Let's approach this via an example.
//  In addition to the trait `From` which "constructs types from other types", the standard library
//  has a trait `Into`, which does the opposite; it "convert this instance into another type".
//
//      let a = VelocityComponent::from((42.0, 6.9));   // Construct velocity using `From`-trait
//      let b: VelocityComponent = (42.0, 6.9).into();  // Construct velocity using `Into`-trait
//
//  The fun thing here is that the behavior is actually so similar, that the implementation for
//  `Into` for anything that implements `From` can actually always be written as:
//
//      impl Into<TargetType> for SourceType {
//          fn into(self) -> TargetType {
//              TargetType::from(self)
//          }
//      }
//
//  If you need to implement both traits for multiple types, this gets infuriating quite fast. Also,
//  this severely violates DRY (Don't Repeat Yourself).
//
//  Blanket implementations to the rescue!
//
//  Luckily, if you just implement `From`, you'll never have to implement `Into` as standard library
//  has blanket implementation to do that. Somewhere in the standard library, there is written
//  something like
//
//      // Here `S` stands for "SourceType" and `T` for "TargetType"
//      impl<S: From<T>, T> Into<T> for S {
//          fn into(self) -> T {
//              T::from(self)
//          }
//      }
//
//  Which automatically implements `Into<T>` for anything that implements `From<T>`. This uses trait
//  bounds to indicate that the source type `S` should always be something that implements from for
//  `T` and then implements `Into<T>` for those. In other words: this tells the compiler that
//
//      "Anything that has the trait `From` also has the trait `Into`"
//
impl From<(f64, f64)> for VelocityComponent {
    fn from(source: (f64, f64)) -> Self {
        VelocityComponent { x: source.0, y: source.1 }
    }
}


struct FrictionComponent {
    amount: f64,
}

impl From<f64> for FrictionComponent {
    fn from(source: f64) -> Self {
        FrictionComponent { amount: source }
    }
}

struct AccelerationComponent {
    x: f64,
    y: f64,
}

impl From<(f64, f64)> for AccelerationComponent {
    fn from(source: (f64, f64)) -> Self {
        AccelerationComponent { x: source.0, y: source.1 }
    }
}


pub fn main() {
    // Init components, but instead of direct constructor calls, use `From` implementations
    let mut positions = vec![
        PositionComponent::from((0.0, 0.0)),
        PositionComponent::from((-42.0, -42.0)),
        PositionComponent::from((234.0, 123.0)),
        PositionComponent::from((6.0, 9.0)),
    ];
    let mut velocities = vec![
        VelocityComponent::from((40.0, 10.0)),
        VelocityComponent::from((30.0, 20.0)),
        VelocityComponent::from((20.0, 30.0)),
        VelocityComponent::from((10.0, 40.0)),
    ];
    let frictions = vec![
        FrictionComponent::from(1.0),
        FrictionComponent::from(2.0),
        FrictionComponent::from(3.0),
        FrictionComponent::from(4.0),
    ];
    let accelerations = vec![
        AccelerationComponent::from((2.0, 16.0)),
        AccelerationComponent::from((4.0, 2.0)),
        AccelerationComponent::from((8.0, 4.0)),
        AccelerationComponent::from((16.0, 8.0)),
    ];

    // Everything is passed to functions by reference. Technically, this is called "borrowing", as
    // we do not actually pass down the ownership, but rather we say "hey, here is a reference to
    // this thing I own. You are allowed to do something with it, but I want it back when you are
    // done with it, OK?". Thus, when the function ends, the reference's lifetime ends, the vector
    // is no longer borrowed and we are again allowed to use it. (For more information on ownership
    // references and borrowing, refer to the chapter 4.)
    //
    // For example, here we borrow `velocities` as mutable and `accelerations` as immutable. When
    // the function ends, references' lifetimes end (because the references go out of scope), thus
    // the borrow ends and...
    apply_acceleration(&mut velocities, &accelerations);
    // ...we are allowed to borrow the `velocities` again as it is no longer borrowed.
    apply_friction(&mut velocities, &frictions);
    apply_velocity(&mut positions, &velocities);

    print_positions(&positions)
}

fn apply_acceleration(velocities: &mut Vec<VelocityComponent>, accelerations: &Vec<AccelerationComponent>) {
    let mut vel_iter = velocities.iter_mut();
    let mut acc_iter = accelerations.iter();

    while let (Some(vel), Some(acc)) = (vel_iter.next(), acc_iter.next()) {
        vel.x += acc.x;
        vel.y += acc.y;
    }
}

// (Logic copied from Konna's `ApplyFrictionSystem`)
fn apply_friction(velocities: &mut Vec<VelocityComponent>, frictions: &Vec<FrictionComponent>) {
    let mut vel_iter = velocities.iter_mut();
    let mut fri_iter = frictions.iter();

    while let (Some(vel), Some(fri)) = (vel_iter.next(), fri_iter.next()) {
        let velocity_length_squared = vel.x * vel.x + vel.y * vel.y;

        // Good-to-know™ (off-topic, but mandatory):
        //  As of writing `f64::EPSILON` is part of an unstable feature "assoc_int_consts". This
        //  causes compiler to fail on this line by default. The feature has passed review process
        //  and is due to "be merged soon", but for now, it is unstable.
        //
        //  To use unstable features, one needs to use "nightly" version of the rust compiler and
        //  add "#[feature(assoc_int_consts)]" to the top of the `main.rs` (or `lib.rs`). Nightly
        //  compiler can be easily installed using `rustup`:
        //      rustup toolchain install nightly
        //
        //  and then changing to use the nightly toolchain version:
        //      rustup default nightly
        //
        if velocity_length_squared < f64::EPSILON {
            continue;
        }

        // Note that `.sqrt()` is an associated function of `f64` instead of some utility class like
        // Java's `Math.sqrt()`. A bit unintuitively, the call DOES NOT MODIFY THE ORIGINAL, though.
        let velocity_length = velocity_length_squared.sqrt();

        // ...same thing for `.abs()`
        let abs_friction_x = (vel.x / velocity_length * fri.amount).abs();
        let abs_friction_y = (vel.y / velocity_length * fri.amount).abs();

        // ...and `.max()`
        let magnitude_x = (vel.x.abs() - abs_friction_x).max(0.0);
        let magnitude_y = (vel.x.abs() - abs_friction_y).max(0.0);

        // ...and `.signum()`
        vel.x = vel.x.signum() * magnitude_x;
        vel.y = vel.y.signum() * magnitude_y;
    }
}

fn apply_velocity(positions: &mut Vec<PositionComponent>, velocities: &Vec<VelocityComponent>) {
    let mut pos_iter = positions.iter_mut();
    let mut vel_iter = velocities.iter();

    while let (Some(pos), Some(vel)) = (pos_iter.next(), vel_iter.next()) {
        pos.x += vel.x;
        pos.y += vel.y;
    }
}

fn print_positions(positions: &Vec<PositionComponent>) {
    let mut value_iter = positions.iter();

    while let Some(value) = value_iter.next() {
        println!("Position: ({},{})", value.x, value.y)
    }
}
