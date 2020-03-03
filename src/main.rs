/*
Step 0. "Just the minimal something that gets the job done"
Topics: Structs, Vectors, (a bit less than The Basics of) Iterators, Shadowing
        inline pattern matching (while-let),

What do we begin with:
    -   main-function
    -   2 component structs
    -   a few iterators
    -   a few while-let -loops

Notes:
    We have a collection of positions and another for velocities. We would like to translate the
    positions by the velocities so that `pos[i] = pos[i] + vel[i]`, where `i` is an index in range
    `0..n`, with `n` being the number of components in each collection.

    Let's get to work: Initialize the component vectors, create iterators and then just iterate and
    perform any data manipulation required. Easy, right?

    Now, while this "works", there are a number of limitations here
        1.  Iterators have to be initialized separately, creating a lot of clutter
        2.  Even with while-let, the loop is mighty ugly and with more than two components, it could
            get unwieldy quite quick. On the other hand if we could use actual iterators, that would
            allow using `.filter()`, `.map()`, `.fold()`, etc. on the component collections. Is that
            useful? I have no clue, but that would be neat!
        3.  Later down the line, when we want to parallelize things, raw vectors are not going to
            cut it anymore.
*/

// Here we define a `struct`. Struct is a simple collection of data. The variables inside the struct
// are called "fields". Each field must have a name and a type.
// (For more info on structs, refer to chapter 5.)
struct PositionComponent {
    x: f64,
    y: f64,
}

struct VelocityComponent {
    x: f64,
    y: f64,
}

pub fn main() {
    // Init components (For more info on common collections, refer to chapter 8.1)
    let mut positions = vec![
        // This is "basic constructor" syntax for structs. In rust there are no language level
        // "constructors" like Java, C++, C# and many other languages do. All structs can be
        // constructed by default like this. If more complex constructing logic is needed, one can
        // define a "constructor" as an associated function like this:
        //
        //      struct MyStruct { /* fields */ }
        //
        //      impl MyStruct {
        //          // Note there is no `self` parameter, this is called an "associated function"
        //          pub fn constructor_name() -> MyStruct {
        //              MyStruct { /* fields */ }
        //          }
        //      }
        //
        //      // Now we can construct a MyStruct using
        //      let my_struct = MyStruct::constructor_name();
        //
        // But, as we are not doing anything too complex here, it is sufficient to just use the
        // regular struct instantiation. (For more info on structs, refer to chapter 5.)
        PositionComponent { x: 0.0, y: 0.0 },
        PositionComponent { x: -42.0, y: -42.0 },
        PositionComponent { x: 234.0, y: 123.0 },
        PositionComponent { x: 6.0, y: 9.0 },
    ];
    let velocities = vec![
        VelocityComponent { x: 40.0, y: 10.0 },
        VelocityComponent { x: 30.0, y: 20.0 },
        VelocityComponent { x: 20.0, y: 30.0 },
        VelocityComponent { x: 10.0, y: 40.0 },
    ];

    // Get iterators for component vectors (For more info on iterators, refer to chapter 13.2)
    let mut pos_iter = positions.iter_mut();
    // Velocity iterator itself need to be mutable, even though the data it refers to is immutable.
    // This is due to the fact that the `.next()` call mutates the iterator's state in order to
    // proceed to the next element. In other words, the data is immutable, the iterator is not.
    let mut vel_iter = velocities.iter();

    // Iterate as long as both return `Some(x)`. When either return `None`, the execution will break
    // out of the loop. This is "convenience inline pattern matching", so-called "if-let"- or
    // "while-let" -syntax. (For more info, refer to chapter 18.)
    while let (Some(pos), Some(vel)) = (pos_iter.next(), vel_iter.next()) {
        pos.x += vel.x;
        pos.y += vel.y;
    }

    // Iterators have been exhausted and cannot be re-used. Shadow the old iterators by creating new
    // ones with same names. Shadowing is just a fancy name for "hiding" the old variable by
    // creating a new one with same name. (For more info, refer to chapter 3.1.)
    let mut value_iter = positions.iter_mut();

    // Again, use while-let to iterate until `None` occurs for the first time
    while let Some(value) = value_iter.next() {
        println!("Position: ({},{})", value.x, value.y)
    }
}
