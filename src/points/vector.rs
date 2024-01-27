use crate::approx::approx;
use core::ops::{Add, Sub};

#[derive(Debug)]
pub struct Vector {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
}

impl Vector {
    pub fn negate(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        approx(self.x, other.x) && approx(self.y, other.y) && approx(self.z, other.z)
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
