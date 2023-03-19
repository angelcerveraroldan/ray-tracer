use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub time: f64,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, time: f64) -> Self {
        Self { p, normal, time }
    }
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}