use enum_dispatch::enum_dispatch;
use sphere::Sphere;

use crate::{
    intersection::{intersections::IntersectionTracker, single_intersection::SingleIntersection},
    ray::Ray,
};

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
    fn get_intersections(&self, ray: &Ray) -> IntersectionTracker {
        IntersectionTracker::new(
            self.hit_times(ray)
                .into_iter()
                .map(|time| SingleIntersection::new(time, self))
                .collect(),
        )
    }
}

#[cfg(test)]
mod test_shapes {
    use super::*;

    fn make_shape() -> Shapes {
        Shapes::Sphere(sphere::Sphere::default())
    }

    #[test]
    fn basic_hits() {
        let shape = make_shape();
        let ray = crate::ray::Ray::from(((0, 0, -5), (0, 0, 1)));
        let hits = shape.get_intersections(&ray);
        assert_eq!(hits[0].time, 4.0);
        assert_eq!(hits[1].time, 6.0);
        assert_eq!(hits[0].shape, &shape);
    }

    #[test]
    fn tangent_intersection() {
        let shape = make_shape();
        let ray = crate::ray::Ray::from(((0, 1, -5), (0, 0, 1)));
        let hits = shape.get_intersections(&ray);
        assert_eq!(hits[0].time, 5.0);
        assert_eq!(hits[1].time, 5.0);
        assert_eq!(hits[0].shape, &shape);
    }
}
