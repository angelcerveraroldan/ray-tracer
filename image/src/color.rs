use crate::{errors::ImageErrors, utils};

pub struct RGBAColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl RGBAColor {
    fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
}

impl From<(u8, u8, u8, u8)> for RGBAColor {
    fn from(value: (u8, u8, u8, u8)) -> Self {
        Self::new(value.0, value.1, value.2, value.3)
    }
}

impl TryFrom<(f64, f64, f64, f64)> for RGBAColor {
    type Error = ImageErrors;
    fn try_from((rp, gp, bp, ap): (f64, f64, f64, f64)) -> Result<Self, Self::Error> {
        let red = utils::rgb_percentage_to_u8(rp)?;
        let green = utils::rgb_percentage_to_u8(gp)?;
        let blue = utils::rgb_percentage_to_u8(bp)?;
        let alpha = utils::rgb_percentage_to_u8(ap)?;

        Ok(Self::from((red, green, blue, alpha)))
    }
}
