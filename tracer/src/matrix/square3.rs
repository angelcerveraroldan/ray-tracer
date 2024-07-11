use super::SquareMatrix;

pub type Matrix3x3 = SquareMatrix<3>;

#[macro_export]
macro_rules! matrix_3x3 {
    ($($($element:expr),*;)*) => {
        crate::matrix::square3::Matrix3x3::from(vec![$( vec![ $($element as f64),* ] ), *])
    };
    ($($($element:expr),*);*) => {
        crate::matrix::square3::Matrix3x3::from(vec![$( vec![ $($element as f64),* ] ), *])
    };
}

impl Matrix3x3 {
    fn remove_indexes(&self, row: usize, col: usize) -> crate::matrix::square2::Matrix2x2 {
        let mut data = self.data.clone();
        data.remove(row);
        for r in data.iter_mut() {
            r.remove(col);
        }
        crate::matrix::square2::Matrix2x2::from(data)
    }
}

#[cfg(test)]
mod matrix_3x3_test {
    use super::*;
    use crate::matrix_2x2;

    #[test]
    fn macro_test() {
        let m1 = matrix_3x3![
            1, 2, 3;
            1, 2, 3;
            1, 2, 3];

        let m2 = Matrix3x3::new([[1.0, 2.0, 3.0], [1.0, 2.0, 3.0], [1.0, 2.0, 3.0]]);

        assert_eq!(m1, m2);

        let m1 = matrix_3x3![
            1, 2, 3;
            1, 2, 3;
            1, 2, 3;
        ];

        assert_eq!(m1, m2);
    }
    #[test]
    fn submatrix() {
        let i = matrix_3x3![
                    1, 5, 0;
                    -3, 2, 7;
                    0, 6, -3
        ];
        let o = matrix_3x3![
                    1, 5, 0;
                    -3, 2, 7;
                    0, 6, -3
        ];

        assert_eq!(i.remove_indexes(0, 0), matrix_2x2![2, 7; 6, -3]);
        assert_eq!(i.remove_indexes(0, 1), matrix_2x2![-3, 7; 0, -3]);
        assert_eq!(i.remove_indexes(2, 1), matrix_2x2![1, 0; -3, 7]);
    }
}
