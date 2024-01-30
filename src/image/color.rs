use std::u8;

/// RGB Percentages.
#[derive(Default, Clone, Debug)]
pub struct Color {
    red_percent: f64,
    green_percent: f64,
    blue_percent: f64,
}

impl Color {
    /// Convert from percentage to rgb.
    ///
    /// This will give an error if any of the three percentages are
    /// larger than one, or smaller than 0.
    pub fn to_rgb(&self) -> Option<(u8, u8, u8)> {
        let r = scale(self.red_percent)?;
        let g = scale(self.green_percent)?;
        let b = scale(self.blue_percent)?;
        Some((r, g, b))
    }
}

fn scale(f: f64) -> Option<u8> {
    if 0.0 <= f && f <= 1.0 {
        Some((f * 256_f64).round() as u8)
    } else {
        None
    }
}

#[cfg(test)]
mod color_test {
    use super::scale;

    #[test]
    fn scale_test() {
        let actual = [0.0, 0.1, 1.0].map(|x| scale(x).unwrap());
        let expected = [0, 26, 255];
        assert_eq!(actual, expected)
    }
}
