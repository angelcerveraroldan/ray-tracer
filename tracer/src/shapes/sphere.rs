use std::ops::Mul;

use crate::{
    matrix::square4::Matrix4x4, point::vector::Vector, ray::Ray, shapes::sphere,
    transformations::TransformationMatrix,
};

use super::Hittable;

#[derive(Debug, PartialEq, Default)]
pub struct Sphere {
    pub transformation: TransformationMatrix,
}

impl Hittable for Sphere {
    fn hit_times(&self, ray: &Ray) -> Vec<f64> {
        let ray = self
            .transformation
            .inverse()
            .map(|inverse| inverse.mul(ray))
            .expect("Could not get inverse of transformation matrix");

        let sphere_ray = Vector::from((ray.origin.x, ray.origin.y, ray.origin.z));
        let a = ray.dir.dot(&ray.dir);
        let b = 2.0 * ray.dir.dot(&sphere_ray);
        let c = sphere_ray.dot(&sphere_ray) - 1.0;
        let discriminant = (b * b) - (4.0 * a * c);
        if discriminant < 0.0 {
            return vec![];
        }
        let sqrt = discriminant.sqrt();
        vec![(-b - sqrt) / (2.0 * a), (-b + sqrt) / (2.0 * a)]
    }
}

#[cfg(test)]
mod test_hittable_sphere {
    use super::*;
    use crate::point::coord::Coord;
    use std::vec;

    #[test]
    fn basic_intersection() {
        let ray = crate::ray::Ray::from(((0, 0, -5), (0, 0, 1)));
        let hits = Sphere::default().hit_times(&ray);
        assert_eq!(hits, vec![4.0, 6.0]);
    }

    #[test]
    fn tangent_intersection() {
        let ray = crate::ray::Ray::from(((0, 1, -5), (0, 0, 1)));
        let hits = Sphere::default().hit_times(&ray);
        assert_eq!(hits, vec![5.0, 5.0]);
    }

    #[test]
    fn no_intersection() {
        let ray = crate::ray::Ray::from(((0, 2, -5), (0, 0, 1)));
        let hits = Sphere::default().hit_times(&ray);
        assert!(hits.is_empty())
    }

    #[test]
    fn inside_intersection() {
        let ray = crate::ray::Ray::from(((0, 0, 0), (0, 0, 1)));
        let hits = Sphere::default().hit_times(&ray);
        assert_eq!(hits, vec![-1.0, 1.0]);
    }

    #[test]
    fn behind_intersection() {
        let ray = crate::ray::Ray::from(((0, 0, 5), (0, 0, 1)));
        let hits = Sphere::default().hit_times(&ray);
        assert_eq!(hits, vec![-6.0, -4.0]);
    }

    #[test]
    fn transform_translate() {
        let mut sphere = Sphere::default();
        sphere.transformation.translate((2, 3, 4));
        assert_eq!(
            sphere.transformation.matrix,
            TransformationMatrix::translation(Coord::from((2, 3, 4)))
        );
    }

    #[test]
    fn intersect_scaled_sphere() {
        let mut sphere = Sphere::default();
        sphere.transformation.scale((2, 2, 2));
        let ray = crate::ray::Ray::from(((0, 0, -5), (0, 0, 1)));
        let hits = sphere.hit_times(&ray);
        assert_eq!(hits, vec![3.0, 7.0]);
    }

    #[test]
    fn intersect_translated_sphere() {
        let mut sphere = Sphere::default();
        sphere.transformation.translate((5, 0, 0));
        let ray = crate::ray::Ray::from(((0, 0, -5), (0, 0, 1)));
        let hits = sphere.hit_times(&ray);
        assert_eq!(hits, vec![]);
    }
}
