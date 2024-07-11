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

impl Matrix2x2 {
    pub fn det(&self) -> f64 {
        self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)]
    }
}

#[cfg(test)]
mod matrix_2x2_test {
    use crate::approx::approx;

    #[test]
    fn test_determinant() {
        assert!(approx(matrix_2x2![1, 5; -3, 2].det(), 17.0));
        assert!(approx(matrix_2x2![8,  -4; 6, 4].det(), 56.0));
        assert!(approx(matrix_2x2![-5, -4; 2, 4].det(), -12.0));
        assert!(approx(matrix_2x2![-5,  8; 2, 6].det(), -46.0));
    }
}
