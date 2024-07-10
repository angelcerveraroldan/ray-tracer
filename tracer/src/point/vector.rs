use crate::approx::approx;
use core::{
    f64,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Clone)]
pub struct Vector {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
}

impl Vector {
    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        Self::new(self.x / magnitude, self.y / magnitude, self.z / magnitude)
    }

    fn dot(&self, rhs: &Vector) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    fn cross_product(&self, rhs: &Vector) -> Vector {
        Vector::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
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

crate::pointCommons!(Vector);

#[cfg(test)]
mod vector_test {
    use crate::approx::approx;

    use super::Vector;

    #[test]
    fn vec_magnitude() {
        let actual_magnitudes = [(1, 0, 0), (0, 1, 0), (0, 0, 1), (1, 2, 3), (-1, -2, -3)]
            .map(|value| Vector::from(value).magnitude());

        let expected_magnitudes = [1.0, 1.0, 1.0, 14.0_f64.sqrt(), 14.0_f64.sqrt()];

        assert_eq!(actual_magnitudes, expected_magnitudes)
    }

    #[test]
    fn vec_normalize() {
        let actual_normal = [(4, 0, 0), (1, 2, 3)].map(|value| Vector::from(value).normalize());
        let expected_normal = [
            (1.0, 0.0, 0.0),
            (
                1.0 / (14_f64).sqrt(),
                2.0 / (14_f64).sqrt(),
                3.0 / (14_f64).sqrt(),
            ),
        ]
        .map(Vector::from);

        assert_eq!(actual_normal, expected_normal)
    }

    // Todo: Numbers should be random
    #[test]
    fn normalized_magnitude() {
        assert!(approx(
            Vector::new(134.1235, -43.234, -89.32)
                .normalize()
                .magnitude(),
            1.0
        ))
    }

    #[test]
    fn scalar_mult() {
        assert_eq!(
            Vector::new(1.3, -143.789, 1234.2) * 2.4,
            Vector::new(1.3 * 2.4, -143.789 * 2.4, 1234.2 * 2.4),
        )
    }

    #[test]
    fn scalar_div() {
        assert_eq!(
            Vector::new(1.3, -143.789, 1234.2) / 2.4,
            Vector::new(1.3 / 2.4, -143.789 / 2.4, 1234.2 / 2.4),
        )
    }

    #[test]
    fn dot_product() {
        let actual_dot_products = [
            ((1.0, 2.0, 3.0), (2.0, 3.0, 4.0)),
            ((1.34, 53.34, -82.1234), (98.234, -0.324, 1.909)),
            ((1.0, 0.0, 0.0), (1.0, 0.0, 0.0)),
        ]
        .map(|(lhs, rhs)| Vector::from(lhs).dot(&Vector::from(rhs)));

        let expected_dot_products = [20.0, -42.4221706, 1.0];

        for (&actual, expected) in actual_dot_products.iter().zip(expected_dot_products) {
            assert!(approx(actual, expected))
        }
    }

    #[test]
    fn cross_product() {
        let actual_cross_products = [
            ((1.0, 2.0, 3.0), (2.0, 3.0, 4.0)),
            ((2.0, 3.0, 4.0), (1.0, 2.0, 3.0)),
        ]
        .map(|(lhs, rhs)| Vector::from(lhs).cross_product(&Vector::from(rhs)));

        let expected_cross_products = [(-1.0, 2.0, -1.0), (1.0, -2.0, 1.0)].map(Vector::from);

        assert_eq!(actual_cross_products, expected_cross_products);
    }
}
