use crate::{
    point::{coord::Coord, vector::Vector},
    ray::Ray,
};

pub struct Sphere {}

impl Sphere {
    pub fn intersection_at(ray: &Ray) -> (Option<f64>, Option<f64>) {
        let sphere_ray = Vector::from((ray.origin.x, ray.origin.y, ray.origin.z));
        let a = ray.dir.dot(&ray.dir);
        let b = 2.0 * ray.dir.dot(&sphere_ray);
        let c = sphere_ray.dot(&sphere_ray) - 1.0;
        let disriminant = (b * b) - (4.0 * a * c);
        if disriminant < 0.0 {
            return (None, None);
        }
        let sqrt = disriminant.sqrt();
        (Some((-b - sqrt) / (2.0 * a)), Some((-b + sqrt) / (2.0 * a)))
    }
}

#[cfg(test)]
mod test_sphere {
    use super::Sphere;

    #[test]
    fn basic_intersection() {
        let ray = crate::ray::Ray::from(((0, 0, -5), (0, 0, 1)));
        let (a, b) = Sphere::intersection_at(&ray);
        assert_eq!(Some(4.0), a);
        assert_eq!(Some(6.0), b);
    }

    #[test]
    fn tangent_intersection() {
        let ray = crate::ray::Ray::from(((0, 1, -5), (0, 0, 1)));
        let (a, b) = Sphere::intersection_at(&ray);
        assert_eq!(Some(5.0), a);
        assert_eq!(Some(5.0), b);
    }

    #[test]
    fn no_intersection() {
        let ray = crate::ray::Ray::from(((0, 2, -5), (0, 0, 1)));
        let (a, b) = Sphere::intersection_at(&ray);
        assert_eq!(None, a);
        assert_eq!(None, b);
    }

    #[test]
    fn inside_intersection() {
        let ray = crate::ray::Ray::from(((0, 0, 0), (0, 0, 1)));
        let (a, b) = Sphere::intersection_at(&ray);
        assert_eq!(Some(-1.0), a);
        assert_eq!(Some(1.0), b);
    }

    #[test]
    fn behind_intersection() {
        let ray = crate::ray::Ray::from(((0, 0, 5), (0, 0, 1)));
        let (a, b) = Sphere::intersection_at(&ray);
        assert_eq!(Some(-6.0), a);
        assert_eq!(Some(-4.0), b);
    }
}
