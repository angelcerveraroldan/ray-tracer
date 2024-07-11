use super::*;

pub type Matrix2x2 = SquareMatrix<2>;

#[macro_export]
macro_rules! matrix_2x2 {
    ($($($element:expr),*;)*) => {
        crate::matrix::square2::Matrix2x2::from(vec![$( vec![ $($element as f64),* ] ), *])
    };
    ($($($element:expr),*);*) => {
        crate::matrix::square2::Matrix2x2::from(vec![$( vec![ $($element as f64),* ] ), *])
    };
}

impl Matrix2x2 {}
