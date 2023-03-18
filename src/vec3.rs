use std::fmt;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

pub type Point3 = Vec3;
pub type RGBColor = Vec3;

/// A 3 coordinate vector
/// Used for directions, rgb, points, etc...
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    vals: [f64; 3],
}


impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            vals: [x, y, z],
        }
    }

    pub fn x(self) -> f64 { self[0] }
    pub fn y(self) -> f64 { self[1] }
    pub fn z(self) -> f64 { self[2] }

    pub fn dot(self, rhs: Vec3) -> f64 { self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2] }

    pub fn magnitude(self) -> f64 { self.dot(self).sqrt() }

    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            vals: [
                self[1] * rhs[2] - self[2] * rhs[1],
                self[2] * rhs[0] - self[0] * rhs[2],
                self[0] * rhs[1] - self[1] * rhs[0]
            ]
        }
    }

    pub fn normalized(self) -> Vec3 { self / self.magnitude() }

    pub fn fmt_color(self) -> String {
        format!("{} {} {}",
                (255.999 * self.x()) as u64,
                (255.999 * self.y()) as u64,
                (255.999 * self.z()) as u64
        )
    }
}

impl From<[f64; 3]> for Vec3 {
    fn from(e: [f64; 3]) -> Self {
        Self { vals: e }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.vals[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.vals[index]
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(
            self.vals
                .zip(rhs.vals)
                .map(|(x, y)| x + y)
        )
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        let added = *self + rhs;
        *self = added;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from(
            self.vals.zip(rhs.vals).map(|(x, y)| x - y)
        )
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        let added = *self - rhs;
        *self = added;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, lambda: f64) -> Self::Output {
        Vec3::from(self.vals.map(|x| x * lambda))
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, lambda: f64) {
        *self = *self * lambda
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::from(
            self.vals.zip(rhs.vals).map(|(x, y)| x * y)
        )
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, lambda: f64) -> Self::Output {
        Vec3::from(
            self.vals.map(|x| x / lambda)
        )
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, lambda: f64) {
        *self = *self / lambda
    }
}


impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self[0], self[1], self[2])
    }
}

#[cfg(test)]
mod test {
    use self::super::*;

    const TESTING_VEC1: Vec3 = Vec3 { vals: [1.0, 3.0, 8.0] };
    const TESTING_VEC2: Vec3 = Vec3 { vals: [2.0, 2.0, 2.0] };
    // TestingVec: 1 + 2
    const TESTING_VEC3: Vec3 = Vec3 { vals: [3.0, 5.0, 10.0] };
    // TestingVec: 1 * 2
    const TESTING_VEC4: Vec3 = Vec3 { vals: [2.0, 6.0, 16.0] };
    // TestingVec: 1 - 2
    const TESTING_VEC5: Vec3 = Vec3 { vals: [-1.0, 1.0, 6.0] };


    #[test]
    fn from_arr() {
        assert_eq!(
            Vec3::from([1.0, 3.0, 8.0]), TESTING_VEC1
        );
    }

    // Addition tests
    #[test]
    fn add() {
        assert_eq!(TESTING_VEC1 + TESTING_VEC2, TESTING_VEC3)
    }

    #[test]
    fn add_assign() {
        let mut x = TESTING_VEC1;
        x.add_assign(TESTING_VEC2);

        assert_eq!(x, TESTING_VEC3)
    }

    // Subtraction tests
    #[test]
    fn subtract() {
        assert_eq!(TESTING_VEC1 - TESTING_VEC2, TESTING_VEC5)
    }

    #[test]
    fn subtract_assign() {
        let mut x = TESTING_VEC1;
        x.sub_assign(TESTING_VEC2);

        assert_eq!(x, TESTING_VEC5)
    }


    // Multiplication tests

    // 1 - Scalar multiplication
    #[test]
    fn vector_mul_scalar() {
        assert_eq!(TESTING_VEC3 * 2.0, Vec3::new(6.0, 10.0, 20.0))
    }

    #[test]
    fn vector_mul_scalar_assign() {
        let mut x = TESTING_VEC3;
        x.mul_assign(2.0);

        assert_eq!(x, Vec3::new(6.0, 10.0, 20.0))
    }

    // 2 - Vec3 multiplication
    #[test]
    fn vector_mul_vector() {
        assert_eq!(TESTING_VEC1 * TESTING_VEC2, TESTING_VEC4)
    }

    // Division Tests
    #[test]
    fn vector_div_scalar() {
        assert_eq!(TESTING_VEC3 / 0.5, Vec3::new(6.0, 10.0, 20.0))
    }

    #[test]
    fn vector_div_scalar_assign() {
        let mut x = TESTING_VEC3;
        x.div_assign(0.5);

        assert_eq!(x, Vec3::new(6.0, 10.0, 20.0))
    }
}