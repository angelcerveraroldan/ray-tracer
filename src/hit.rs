use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub time: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, time: f64) -> Self {
        Self { p, normal, time, front_face: false }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) -> () {
        // Check if the normal and the ray are moving in the same direction
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            outward_normal * (-1.0)
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

// A list of all of the objects that are in the world
pub type ObjectsInWorld = Vec<Box<dyn Hittable>>;

impl Hittable for ObjectsInWorld {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut nearest_hit_record = None;
        let mut time_at_nearest = t_max;

        for object in self {
            // We will always only check between the min time, and the nearest time
            if let Some(hitRecord) = object.hit(ray, t_min, time_at_nearest) {
                time_at_nearest = hitRecord.time;
                nearest_hit_record = Some(hitRecord);
            }
        }

        nearest_hit_record
    }
}
