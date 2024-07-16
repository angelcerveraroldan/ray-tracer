use super::*;

pub type Matrix2x2 = SquareMatrix<2>;

#[macro_export]
macro_rules! matrix_2x2 {
    ($($($element:expr),*;)*) => {
        $crate::matrix::square2::Matrix2x2::from([$( [ $($element as f64),* ] ), *])
    };
    ($($($element:expr),*);*) => {
        $crate::matrix::square2::Matrix2x2::from([$( [ $($element as f64),* ] ), *])
    };
}

impl Matrix2x2 {
    pub fn det(&self) -> f64 {
        self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)]
    }

    pub fn inverse(&self) -> Option<Matrix2x2> {
        let d = self.det();
        if d == 0.0 {
            return None;
        }

        let d = 1.0 / d;
        matrix_2x2![
            d * self.data[1][1], - d * self.data[0][1]; - d * self.data[1][0], d * self.data[0][0]
        ]
        .into()
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

    #[test]
    fn inverse() {
        let m = matrix_2x2![1, 3; -3, 12];
        assert_eq!(
            m.inverse().unwrap(),
            matrix_2x2![4.0/7.0, -1.0/7.0; 1.0/7.0, 1.0/21.0]
        );
    }
}
