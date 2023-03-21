#![feature(array_zip)]

mod vec3;
mod ray;
mod sphere;
mod utils;
mod hit;

use vec3::{Vec3, RGBColor, Point3};
use ray::Ray;
use std::io::{stderr, Write};
use crate::sphere::Sphere;

/// Blend white and blue depending on the y-coord
///
fn ray_color(ray: &Ray) -> RGBColor {
    let sphere = Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
    );

    let hit_time = sphere.ray_collision(ray);

    if hit_time > 0.0 {
        // The normal will be the line from the center to the point of impact
        // This is the same as the point of impact of the ray relative to the center of the ball
        let normal = (ray.at(hit_time) - sphere.center()).normalized();
        return RGBColor::new(
            normal.x() + 1.0,
            normal.y() + 1.0,
            normal.z() + 1.0,
        ) * 0.5;
    }

    // -1 <= x, y, z <= 1  ---  after normalizing
    let unit_dir = ray.direction().normalized();

    // Add 1 to the y direction, now this is between 0 and 2, so multiply by 0.5
    let t = (unit_dir.y() + 1.0) * 0.5;

    (RGBColor::new(1.0, 1.0, 1.0) * (1.0 - t)) + (RGBColor::new(0.5, 0.7, 1.0) * t)
}

fn main() {
    // Image Setup
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = ((256_f64) / ASPECT_RATIO) as u64;

    // Camera Setup
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);


    println!("P3");
    println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {j:3}");
        stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

            let ray = Ray::new(
                origin,
                lower_left_corner + (horizontal * u) + (vertical * v) - origin,
            );

            let pixel_color = ray_color(&ray);

            println!("{}", pixel_color.fmt_color());
        }
    }

    eprint!("\nDone!\n");
}
