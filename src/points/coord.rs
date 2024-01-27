use crate::approx::approx;
use crate::points::vector::Vector;

#[derive(Debug)]
pub struct Coord {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        approx(self.x, other.x) && approx(self.y, other.y) && approx(self.z, other.z)
    }
}

impl Coord {
    // Given two points, genertate the vector that points from the first coord to the second
    pub fn vector_to(self, to: &Coord) -> Vector {
        Vector::new(to.x - self.x, to.y - self.y, to.z - self.z)
    }

    pub fn add_vector(self, vec: &Vector) -> Coord {
        Coord::new(self.x + vec.x, self.y + vec.y, self.z + vec.z)
    }

    pub fn subtract_vector(self, vec: &Vector) -> Coord {
        Coord::new(self.x - vec.x, self.y - vec.y, self.z - vec.z)
    }

    pub fn negate(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}
