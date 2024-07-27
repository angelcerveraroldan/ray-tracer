use std::f64::{self, consts::PI};

use image::{canvas::Canvas, color};
use point::coord::Coord;
use transformations::Transform;

pub(crate) mod approx;
pub(crate) mod matrix;
pub(crate) mod point;
pub(crate) mod ray;
pub(crate) mod shapes;
pub(crate) mod transformations;

const H: usize = 1000;
const W: usize = 1000;

fn to_pixel(Coord { x, y, .. }: Coord) -> (usize, usize) {
    let h = ((H - 1) as f64 / 2.0).floor();
    let w = ((W - 1) as f64 / 2.0).floor();

    let center_y = H as f64 / 2.;
    let center_x = W as f64 / 2.;

    let x = center_x - (w * x);
    let y = center_y - (h * y);

    (x as usize, y as usize)
}

fn draw_circle(canvas: &mut Canvas, (x, y): (usize, usize), rad: usize) {
    let x = x as isize;
    let y = y as isize;
    let rad = rad as isize;

    for row in y - rad..y + rad {
        if row < 0 {
            continue;
        }
        for col in x - rad..x + rad {
            if col < 0 {
                continue;
            }

            let ds = (row - y).abs().pow(2) + (col - x).abs().pow(2);
            if ds <= rad * rad {
                let red = color::RGBAColor::from((1.0, row as f64 / (y + 2 * rad) as f64, 0.0));
                canvas.set_pixel_color((row as usize, col as usize), red);
            }
        }
    }
}

// Draw a clock using the image api and the trasformations
fn main() {
    let mut canvas = image::canvas::Canvas::with_size(H, W);

    // rad in pixels
    let rad = 10;

    let mut p = Coord::from((0., 0.5, 0.));

    for _ in 0..12 {
        println!("{:?}", p);
        draw_circle(&mut canvas, to_pixel(p.clone()), rad);
        p = p.rotate(transformations::Axis::Z, PI / 6.0);
    }

    canvas.save_as_ppm("../test_outputs/clock.ppm".into());
}
