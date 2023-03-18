use crate::ray::Ray;
use crate::vec3::Point3;

pub struct Sphere {
    center: Point3,
    rad: f64,
}

impl Sphere {
    pub fn ray_collision(&self, ray: &Ray) -> bool {
        // Position of the ray relative to the sphere
        let relative_position = ray.origin() - self.center;

        // Find values of a, b, c (will be used in the -b formula)
        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * relative_position.dot(ray.direction());
        let c = relative_position.dot(relative_position) - self.rad * self.rad;

        // Whats inside the sqrt
        let discriminant = (b * b) - (4.0 * a * c);

        // If its greater than 0, then there exists time/s of impact
        discriminant > 0.0
    }
    
    pub fn new(center: Point3, rad: f64) -> Self {
        Self { center, rad }
    }
}