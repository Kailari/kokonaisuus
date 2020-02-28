use std::ops::{Add, Mul, AddAssign};
use std::fmt::{Display, Formatter};

// What on earth does "derive" mean? There usually is an easy or straightforward way of implementing
// some common behavior on some new struct. However, writing those trivial implementations is quite
// cumbersome. Derive-annotation allows us to generate some default implementations for common
// traits.
//
// In this case, we implement the copy and clone traits which allow creating simple copies of
// instances. It is easily seen that in this case, the task of copying instances of `Vector` is
// a fairly trivial component-by-component copy task. The code generator is smart enough to figure
// it out. We just have to explicitly tell it to generate the code, and to do that we need the
// derive-annotation.
//
// Refer to chapters 3 and 4 for explanation on what Copy/Clone traits imply.
#[derive(Copy, Clone)]
pub struct Vector2d {
    pub x: f64,
    pub y: f64,
}

// Trusty ol' `From`-implementation, for Vectors this time
impl From<(f64, f64)> for Vector2d {
    fn from(source: (f64, f64)) -> Self {
        Vector2d { x: source.0, y: source.1 }
    }
}

// Implement `Display` for vectors. This allows nice formatting when printing out.
impl Display for Vector2d {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.3}, {:.3})", self.x, self.y)
    }
}

// Implement add for vectors. Implementation performs component-wise sum.
// This allows the use of the `+` operator on two vectors.
impl Add for Vector2d {
    type Output = Vector2d;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// Implement adding assignment for vectors. This allows doing `vec_a += vec_b`
impl AddAssign for Vector2d{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

// Implement scalar multiplication. This allows `let vec_b = vec_a * 42.0`
impl Mul<f64> for Vector2d {
    type Output = Vector2d;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector2d {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

// Wait a minute, `impl` without specifying a trait?
//
// Well, nothing special here, this is the (bit awkward) syntax for specifying methods and
// associated functions for structs. These methods are not associated with any trait, but rather
// the struct itself.
impl Vector2d {
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        Vector2d {
            x: self.x / length,
            y: self.y / length,
        }
    }

    pub fn abs(&self) -> Self {
        Vector2d {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}
