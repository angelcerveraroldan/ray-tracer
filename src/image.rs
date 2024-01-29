use std::ops::{Add, Div, Mul, Sub};

use crate::approx::approx;

pub mod canvas;

#[derive(Debug, Clone)]
pub struct RGBPercentage {
    red: f64,
    green: f64,
    blue: f64,
}

impl RGBPercentage {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }
}

impl Default for RGBPercentage {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl Add for RGBPercentage {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        RGBPercentage::new(
            self.red + rhs.red,
            self.green + rhs.green,
            self.blue + rhs.blue,
        )
    }
}

impl Sub for RGBPercentage {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        RGBPercentage::new(
            self.red - rhs.red,
            self.green - rhs.green,
            self.blue - rhs.blue,
        )
    }
}

impl Mul<f64> for RGBPercentage {
    type Output = RGBPercentage;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl Div<f64> for RGBPercentage {
    type Output = RGBPercentage;
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            red: self.red / rhs,
            green: self.green / rhs,
            blue: self.blue / rhs,
        }
    }
}

impl PartialEq for RGBPercentage {
    fn eq(&self, other: &Self) -> bool {
        approx(self.red, other.red)
            && approx(self.green, other.green)
            && approx(self.blue, other.blue)
    }
}
