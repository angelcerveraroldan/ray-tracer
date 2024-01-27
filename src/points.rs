mod coord;
mod vector;

macro_rules! multiplication {
    ( $lhs:ty ) => {
        impl $lhs {
            pub fn negate(self) -> Self {
                <$lhs>::new(-self.x, -self.y, -self.z)
            }

            pub fn new(x: f64, y: f64, z: f64) -> Self {
                Self { x, y, z }
            }
        }
    };
}

#[cfg(test)]
mod coords_testing {
    use crate::points::coord::*;
    use crate::points::vector::*;

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

    const VEC3: Vector = Vector {
        x: 65.3617525973,
        y: 11.7450001457,
        z: 19.5928891141,
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
