#![feature(array_zip)]

mod vec3;
mod ray;
mod sphere;
mod utils;
mod hit;
mod camera;

use vec3::{Vec3, RGBColor, Point3};
use ray::Ray;
use hit::{Hittable, ObjectsInWorld};
use std::io::{stderr, Write};
use rand::Rng;
use crate::camera::Camera;
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
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PP: u64 = 100;

    // World Setup
    let mut world: ObjectsInWorld = ObjectsInWorld::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera Setup
    let camera = Camera::new();

    println!("P3");
    println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
    println!("255");

    let mut range = rand::thread_rng();

    for y in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {y:3}");
        stderr().flush().unwrap();

        for x in 0..IMAGE_WIDTH {
            let mut pixel_color = RGBColor::new(0.0, 0.0, 0.0);
            for _ in (0..SAMPLES_PP) {
                let random_u: f64 = range.gen();
                let random_v: f64 = range.gen();

                let u = (x as f64 + random_u) / ((IMAGE_WIDTH - 1) as f64);
                let v = (y as f64 + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world);
            }

            println!("{}", pixel_color.fmt_color(SAMPLES_PP));
        }
    }

    eprint!("\nDone!\n");
}
