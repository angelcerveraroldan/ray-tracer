use std::ops::{Add, Div, Mul, Sub};

use crate::approx::approx;
use crate::point::vector::Vector;

#[derive(Debug, Clone, Default)]
pub struct Coord {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
}

impl Coord {
    // Given two points, genertate the vector that points from the first coord to the second
    pub fn vector_to(self, to: &Coord) -> Vector {
        Vector::new(to.x - self.x, to.y - self.y, to.z - self.z)
    }

    pub fn add_vector(&self, vec: &Vector) -> Coord {
        Coord::new(self.x + vec.x, self.y + vec.y, self.z + vec.z)
    }

    pub fn subtract_vector(&self, vec: &Vector) -> Coord {
        Coord::new(self.x - vec.x, self.y - vec.y, self.z - vec.z)
    }
}

impl Add<Vector> for Coord {
    type Output = Coord;
    fn add(self, vec: Vector) -> Self::Output {
        Coord::new(self.x + vec.x, self.y + vec.y, self.z + vec.z)
    }
}

impl Sub<Vector> for Coord {
    type Output = Coord;
    fn sub(self, vec: Vector) -> Self::Output {
        Coord::new(self.x - vec.x, self.y - vec.y, self.z - vec.z)
    }
}

crate::pointCommons!(Coord);

#[cfg(test)]
mod coord_test {
    use super::Coord;

    #[test]
    fn scalar_mult() {
        assert_eq!(
            Coord::new(1.3, -143.789, 1234.2) * 2.4,
            Coord::new(1.3 * 2.4, -143.789 * 2.4, 1234.2 * 2.4),
        )
    }

    #[test]
    fn scalar_div() {
        assert_eq!(
            Coord::new(1.3, -143.789, 1234.2) / 2.4,
            Coord::new(1.3 / 2.4, -143.789 / 2.4, 1234.2 / 2.4),
        )
    }
}
