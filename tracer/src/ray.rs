use crate::point::{coord::Coord, vector::Vector};

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
    use crate::point::{coord::Coord, vector::Vector};

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
}
