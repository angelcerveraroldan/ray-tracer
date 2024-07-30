use std::f64::{self, consts::PI};

use image::{
    canvas::Canvas,
    color::{self, RGBAColor},
};
use point::{coord::Coord, vector::Vector};
use ray::Ray;
use shapes::{sphere::Sphere, Hittable};
use transformations::Transform;

pub(crate) mod approx;
pub(crate) mod intersection;
pub(crate) mod matrix;
pub(crate) mod point;
pub(crate) mod ray;
pub(crate) mod shapes;
pub(crate) mod transformations;

const H: usize = 300;
const W: usize = 300;

// Draw a clock using the image api and the trasformations
fn main() {
    let mut canvas = image::canvas::Canvas::with_size(H, W);

    let red = RGBAColor::from((0.5, 0.0, 0.5));
    let mut sphere = Sphere::default();
    sphere
        .transformation
        .scale((100, 100, 1))
        .rotate(transformations::Axis::Y, f64::consts::PI / 4.0)
        .translate((-40, -40, 0));

    for row in 0..H {
        for col in 0..W {
            println!("{row}/{H} - {col}/{W}");

            let ray_origin = Coord::from((
                -(W as f64 / 2.0) + (row as f64),
                (H as f64 / 2.0) - (col as f64),
                -5.0,
            ));

            let ray_direction = Vector::from((0, 0, 1));
            let ray = Ray::new(ray_origin, ray_direction);

            if !sphere.hit_times(&ray).is_empty() {
                canvas.set_pixel_color(
                    (row, col),
                    RGBAColor::from((row as f64 / H as f64, 0.0, col as f64 / W as f64)),
                );
            }
        }
    }

    canvas.save_as_ppm("sphere.ppm".to_string());
}
