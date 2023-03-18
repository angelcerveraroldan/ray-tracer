#![feature(array_zip)]

mod vec3;
mod ray;

use vec3::{Vec3, RGBColor};
use std::io::{stderr, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = 256;

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:3}", j);
        stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let pixel_color = RGBColor::new(
                (i as f64) / ((IMAGE_WIDTH - 1) as f64),
                (j as f64) / ((IMAGE_HEIGHT - 1) as f64),
                0.25,
            );

            println!("{}", pixel_color.fmt_color());
        }
    }

    eprint!("\nDone!\n");
}
