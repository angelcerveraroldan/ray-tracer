use crate::point::{coord::Coord, vector::Vector};

#[derive(Debug, PartialEq)]
pub struct Ray {
    pub origin: Coord,
    pub dir: Vector,
}

impl Ray {
    pub fn new(origin: Coord, dir: Vector) -> Self {
        Self { origin, dir }
    }

    fn position_at(&self, t: f64) -> Coord {
        self.origin.add_vector(&self.dir.scalar_mult(t))
    }
}

impl<A, B> From<(A, B)> for Ray
where
    Coord: From<A>,
    Vector: From<B>,
{
    fn from((fc, fr): (A, B)) -> Self {
        Ray::new(Coord::from(fc), Vector::from(fr))
    }
}

#[cfg(test)]
mod test_ray {
    use crate::{
        point::{coord::Coord, vector::Vector},
        transformations::Transform,
    };

    use super::Ray;

    #[test]
    fn change_position() {
        let o = Coord::from((2, 3, 4));
        let d = Vector::from((1, 0, 0));
        let ray = Ray::new(o, d);

        for i in 0..10 {
            let exp = Coord::from((i + 2, 3, 4));
            assert_eq!(ray.position_at(i as f64), exp);
        }
    }

    const DIR: Vector = Vector {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    const ORIGIN: Coord = Coord {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    const RAY: Ray = Ray {
        dir: DIR,
        origin: ORIGIN,
    };

    #[test]
    fn transform_translate() {
        let new_ray = RAY.translate((3.0, 4.0, 5.0));
        assert_eq!(new_ray, Ray::from(((4.0, 6.0, 8.0), (0.0, 1.0, 0.0))));
    }

    #[test]
    fn transform_scale() {
        let new_ray = RAY.scale((2.0, 3.0, 4.0));
        assert_eq!(new_ray, Ray::from(((2.0, 6.0, 12.0), (0.0, 3.0, 0.0))));
    }
}
