#![feature(array_zip)]

mod vec3;
mod ray;
mod sphere;
mod utils;
mod hit;

use vec3::{Vec3, RGBColor, Point3};
use ray::Ray;
use hit::{Hittable, ObjectsInWorld};
use std::io::{stderr, Write};
use crate::sphere::Sphere;

/// Blend white and blue depending on the y-coord
///
fn ray_color(ray: &Ray, world: &ObjectsInWorld) -> RGBColor {
    // Find the nearest hit, else, render gradient
    if let Some(rec) = world.hit(ray, 0.0, f64::INFINITY) {
        (rec.normal + RGBColor::new(1.0, 1.0, 1.0)) * 0.5
    } else {
        let unit_direction = ray.direction().normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (RGBColor::new(1.0, 1.0, 1.0) * (1.0 - t)) + (RGBColor::new(0.5, 0.7, 1.0) * t)
    }
}

fn main() {
    // Image Setup
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = ((256_f64) / ASPECT_RATIO) as u64;

    // World Setup
    let mut world: ObjectsInWorld = ObjectsInWorld::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera Setup
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);

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
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);


    println!("P3");
    println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
    println!("255");

    for y in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {y:3}");
        stderr().flush().unwrap();

        for x in 0..IMAGE_WIDTH {
            let u = (x as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (y as f64) / ((IMAGE_HEIGHT - 1) as f64);

            let ray = Ray::new(
                origin,
                lower_left_corner + (horizontal * u) + (vertical * v) - origin,
            );

            let pixel_color = ray_color(&ray, &world);

            println!("{}", pixel_color.fmt_color());
        }
    }

    eprint!("\nDone!\n");
}
