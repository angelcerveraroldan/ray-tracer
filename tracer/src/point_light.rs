use image::color::RGBAColor;

use crate::point::coord::Coord;

pub struct PointLight {
    pub color: RGBAColor,
    pub position: Coord,
}
