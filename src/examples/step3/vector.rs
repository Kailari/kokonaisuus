use std::ops::{Add, Mul, AddAssign};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
pub struct Vector2d {
    pub x: f64,
    pub y: f64,
}

impl From<(f64, f64)> for Vector2d {
    fn from(source: (f64, f64)) -> Self {
        Vector2d { x: source.0, y: source.1 }
    }
}

impl Display for Vector2d {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.3}, {:.3})", self.x, self.y)
    }
}

impl Add for Vector2d {
    type Output = Vector2d;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vector2d{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<f64> for Vector2d {
    type Output = Vector2d;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector2d {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

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
