use crate::hit::{Hit, HitRecord};
use crate::ray::Ray;
use crate::vec3::Point3;

pub struct Sphere {
    center: Point3,
    rad: f64,
}

impl Sphere {
    pub fn ray_collision(&self, ray: &Ray) -> f64 {
        // Position of the ray relative to the sphere
        let relative_position = ray.origin() - self.center;

        // Find values of a, b, c (will be used in the -b formula)
        let a = ray.direction().magnitude().powi(2);
        let half_b = relative_position.dot(ray.direction());
        let c = relative_position.magnitude().powi(2) - self.rad * self.rad;

        // Whats inside the sqrt
        let discriminant = (half_b * half_b) - (a * c);

        // No impact
        if discriminant < 0.0 {
            -1.0
        } else {
            // Smallest point of impact (nearest one)
            (-half_b - discriminant.sqrt()) / (a)
        }
    }

    pub fn new(center: Point3, rad: f64) -> Self {
        Self { center, rad }
    }

    pub fn center(&self) -> Point3 {
        self.center
    }
    pub fn rad(&self) -> f64 {
        self.rad
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Position of the ray relative to the sphere
        let relative_position = ray.origin() - self.center;

        // Find values of a, b, c (will be used in the -b formula)
        let a = ray.direction().magnitude().powi(2);
        let half_b = relative_position.dot(ray.direction());
        let c = relative_position.magnitude().powi(2) - self.rad * self.rad;

        // Whats inside the sqrt
        let discriminant = (half_b * half_b) - (a * c);

        // No impact
        if discriminant < 0.0 {
            return None;
        }


        // Now we find the SMALLEST time of impact
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || t_max > root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max > root {
                // The time of interception was too small or large (behind camera or behind another object)
                return None;
            }
        }

        let point_of_impact = ray.at(root);
        /*
            This is always going to have a magnitude of rad, since the vector is from the center of
            the sphere to a point in it's surface
         */
        let normal_at_impact_point = (point_of_impact - self.center) / self.rad;

        let hit_record =
            HitRecord::new(point_of_impact, normal_at_impact_point, root);

        Some(hit_record)
    }
}