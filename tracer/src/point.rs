pub mod coord;
pub mod vector;

// This will implement the following:
//
// Equality (via apporximation)
// Scalar multiplication (right hand side)
// Negation and creation
#[macro_export]
macro_rules! pointCommons {
    ( $typename:ty ) => {
        impl $typename {
            pub fn negate(self) -> Self {
                <$typename>::new(-self.x, -self.y, -self.z)
            }

            pub fn new(x: f64, y: f64, z: f64) -> Self {
                Self { x, y, z }
            }
        }

        impl Mul<f64> for $typename {
            type Output = $typename;
            fn mul(self, rhs: f64) -> Self::Output {
                Self {
                    x: self.x * rhs,
                    y: self.y * rhs,
                    z: self.z * rhs,
                }
            }
        }

        impl Div<f64> for $typename {
            type Output = $typename;
            fn div(self, rhs: f64) -> Self::Output {
                Self {
                    x: self.x / rhs,
                    y: self.y / rhs,
                    z: self.z / rhs,
                }
            }
        }

        impl PartialEq for $typename {
            fn eq(&self, other: &Self) -> bool {
                approx(self.x, other.x) && approx(self.y, other.y) && approx(self.z, other.z)
            }
        }

        // Create type from 3 numbers
        impl<A: Into<f64>> From<(A, A, A)> for $typename {
            fn from(value: (A, A, A)) -> Self {
                Self::new(value.0.into(), value.1.into(), value.2.into())
            }
        }
    };
}

// TODO: These tests are horribly and unecessarily long
#[cfg(test)]
mod coords_testing {
    use crate::point::coord::*;
    use crate::point::vector::*;

    const VEC1: Vector = Vector {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    const VEC2: Vector = Vector {
        x: 35.6005351458,
        y: 8.9719036976,
        z: 25.4859126077,
    };

    const COORD1: Coord = Coord {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    const COORD2: Coord = Coord {
        x: 75.7327360547,
        y: -93.1309536826,
        z: 71.8102055871,
    };

    const COORD3: Coord = Coord {
        x: 41.4919679428,
        y: 37.2744247714,
        z: 49.599311264,
    };

    #[test]
    fn add_and_subtract_vectors() {
        assert_eq!(
            VEC1 + VEC2,
            Vector {
                x: 36.6005351458,
                y: 9.9719036976,
                z: 26.4859126077,
            }
        );

        assert_eq!(
            VEC1 - VEC2,
            Vector {
                x: -34.6005351458,
                y: -7.9719036976,
                z: -24.4859126077,
            }
        );
    }

    #[test]
    fn move_coordinate() {
        assert_eq!(
            COORD2.add_vector(&VEC1),
            Coord {
                x: 76.7327360547,
                y: -92.1309536826,
                z: 72.8102055871,
            }
        );

        assert_eq!(
            COORD2.subtract_vector(&VEC2),
            Coord {
                x: 40.132200908899996,
                y: -102.10285738019999,
                z: 46.3242929794,
            }
        );
    }

    #[test]
    fn generate_vector() {
        assert_eq!(
            COORD3.vector_to(&COORD3),
            Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }
        );

        assert_eq!(
            COORD1.vector_to(&COORD3),
            Vector {
                x: 40.4919679428,
                y: 36.2744247714,
                z: 48.599311264,
            }
        )
    }

    #[test]
    fn negate_points() {
        assert_eq!(VEC2.negate(), Vector::new(0.0, 0.0, 0.0) - VEC2);
        assert_eq!(
            COORD2.negate(),
            Coord {
                x: -75.7327360547,
                y: 93.1309536826,
                z: -71.8102055871,
            }
        );
    }
}
