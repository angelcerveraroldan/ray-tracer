use crate::utils;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct RGBAColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl RGBAColor {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn format_string(&self) -> String {
        format!("{} {} {}", self.red, self.green, self.blue)
    }
}

impl From<(u8, u8, u8, u8)> for RGBAColor {
    fn from(value: (u8, u8, u8, u8)) -> Self {
        Self::new(value.0, value.1, value.2, value.3)
    }
}

impl From<(u8, u8, u8)> for RGBAColor {
    fn from(value: (u8, u8, u8)) -> Self {
        Self::new(value.0, value.1, value.2, 255)
    }
}

impl From<(f64, f64, f64, f64)> for RGBAColor {
    fn from((rp, gp, bp, ap): (f64, f64, f64, f64)) -> Self {
        let errmessage = "RGBA f64 values represent percentage, so they must be between 0 and 1";
        let red = utils::rgb_percentage_to_u8(rp).expect(errmessage);
        let green = utils::rgb_percentage_to_u8(gp).expect(errmessage);
        let blue = utils::rgb_percentage_to_u8(bp).expect(errmessage);
        let alpha = utils::rgb_percentage_to_u8(ap).expect(errmessage);

        Self::from((red, green, blue, alpha))
    }
}

impl From<(f64, f64, f64)> for RGBAColor {
    fn from((rp, gp, bp): (f64, f64, f64)) -> Self {
        Self::from((rp, gp, bp, 1.0))
    }
}
