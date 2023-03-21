use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    /*
        The origin is in the center of the "display", so to get the bottom left,
        we need to go down and left by half of the width, and height

        ***********************************************************
        *                                                         *
        *                                                         *
        *                                                         *
        *                                                         *
        *                                                         *
        *                                                         *
        *                                                         *
        *                         origin                          * ^
        *                                                         * |
        *                                                         * |
        *                                                         * height / 2
        *                                                         * |
        *                                                         * |
        *                                                         * |
        *********************************************************** v
        ^ --------width / 2-------- ^
     */

    pub fn new() -> Camera {
        const ASPECT_RATIO: f64 = 16.0 / 9.0;
        const VIEWPORT_HEIGHT: f64 = 2.0;
        const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
        const FOCAL_LENGTH: f64 = 1.0;

        let orig = Point3::new(0.0, 0.0, 0.0);
        let h = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
        let v = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);

        // Lower left corner
        let llc = orig - h / 2.0 - v / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

        Camera {
            origin: orig,
            lower_left_corner: llc,
            horizontal: h,
            vertical: v,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner
                + (self.horizontal * u)
                + (self.vertical * v)
                - self.origin,
        )
    }
}