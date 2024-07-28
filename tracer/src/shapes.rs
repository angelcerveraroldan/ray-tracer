use enum_dispatch::enum_dispatch;
use sphere::Sphere;

use crate::{intersection::Intersection, intersection::Intersections, ray::Ray};

pub mod sphere;

#[derive(Debug, PartialEq)]
#[enum_dispatch(Hittable)]
pub enum Shapes {
    Sphere(Sphere),
}

#[enum_dispatch]
pub trait Hittable {
    /// Return the times when the shape was struck by the ray
    fn hit_times(&self, ray: &Ray) -> Vec<f64>;
}

impl Shapes {
    fn get_intersections<'a>(&'a self, ray: &Ray) -> Intersections<'a> {
        self.hit_times(ray)
            .into_iter()
            .map(|time| Intersection::new(time, self))
            .collect()
    }
}

#[cfg(test)]
mod test_shapes {
    use super::*;

    const SHAPE: Shapes = Shapes::Sphere(sphere::Sphere {});

    #[test]
    fn basic_hits() {
        let ray = crate::ray::Ray::from(((0, 0, -5), (0, 0, 1)));
        let hits = SHAPE.get_intersections(&ray);
        assert_eq!(hits[0].time, 4.0);
        assert_eq!(hits[1].time, 6.0);
        assert_eq!(hits[0].shape, &SHAPE);
    }

    #[test]
    fn tangent_intersection() {
        let ray = crate::ray::Ray::from(((0, 1, -5), (0, 0, 1)));
        let hits = SHAPE.get_intersections(&ray);
        assert_eq!(hits[0].time, 5.0);
        assert_eq!(hits[1].time, 5.0);
        assert_eq!(hits[0].shape, &SHAPE);
    }
}
