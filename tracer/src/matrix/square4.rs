use square3::Matrix3x3;

use crate::point::{self, coord::Coord};

use super::*;

pub type Matrix4x4 = SquareMatrix<4>;

#[macro_export]
macro_rules! matrix_4x4 {
    ($($($element:expr),*;)*) => {
        crate::matrix::square4::Matrix4x4::from(vec![$( vec![ $($element as f64),* ] ), *])
    };
    ($($($element:expr),*);*) => {
        crate::matrix::square4::Matrix4x4::from(vec![$( vec![ $($element as f64),* ] ), *])
    };
}

impl Mul for Matrix4x4 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut product_matrix = Matrix4x4::default();

        for row in 0..4 {
            for col in 0..4 {
                product_matrix[row][col] = self[row][0] * rhs[0][col]
                    + self[row][1] * rhs[1][col]
                    + self[row][2] * rhs[2][col]
                    + self[row][3] * rhs[3][col];
            }
        }

        product_matrix
    }
}

impl Matrix4x4 {
    fn remove_indexes(&self, row: usize, col: usize) -> crate::matrix::square3::Matrix3x3 {
        let mut data = self.data.clone();
        data.remove(row);
        for r in data.iter_mut() {
            r.remove(col);
        }
        Matrix3x3::from(data)
    }
}

#[cfg(test)]
mod matrix_4x4_test {
    use crate::{approx::approx, matrix_3x3};

    use super::*;
    use crate::matrix::square3::*;
    #[test]
    fn macro_test() {
        let m1 = matrix_4x4![
            1, 2, 3, 4;
            1, 2, 3, 4;
            1, 2, 3, 4;
            1, 2, 3, 4];

        let m2 = Matrix4x4::new([
            [1.0, 2.0, 3.0, 4.0],
            [1.0, 2.0, 3.0, 4.0],
            [1.0, 2.0, 3.0, 4.0],
            [1.0, 2.0, 3.0, 4.0],
        ]);

        assert_eq!(m1, m2);

        let m1 = matrix_4x4![
            1, 2, 3, 4;
            1, 2, 3, 4;
            1, 2, 3, 4;
            1, 2, 3, 4;
        ];

        assert_eq!(m1, m2);
    }

    #[test]
    fn map_matrix() {
        assert_eq!(
            matrix_4x4![
                1, 2, 1, 2;
                3, 4, 3, 4;
                1, 2, 1, 2;
                3, 4, 3, 4]
            .map_elements(|&n| n + 1.0),
            matrix_4x4![
                2, 3, 2, 3;
                4, 5, 4, 5;
                2, 3, 2, 3;
                4, 5, 4, 5]
        )
    }

    #[test]
    fn matrix_multiply() {
        let m1 = matrix_4x4![
            1, 2, 3, 4;
            1, 2, 3, 4;
            1, 2, 3, 4;
            1, 2, 3, 4];

        let m2 = matrix_4x4![
            1, 2, 3, 4;
            1, 2, 3, 4;
            1, 2, 3, 4;
            1, 2, 3, 4];

        let m3 = matrix_4x4![
            10, 20, 30, 40;
            10, 20, 30, 40;
            10, 20, 30, 40;
            10, 20, 30, 40];

        assert_eq!(m1 * m2, m3);

        let m1 = matrix_4x4![
            1, 2, 3, 4;
            5, 6, 7, 8;
            9, 8, 7, 6;
            5, 4, 3, 2];

        let m2 = matrix_4x4![
            -2, 1, 2, 3;
            3, 2, 1, -1;
            4, 3, 6, 5;
            1, 2, 7, 8];

        let m3 = matrix_4x4![
            20, 22, 50, 48;
            44, 54, 114, 108;
            40, 58, 110, 102;
            16, 26, 46, 42];

        assert_eq!(m1 * m2, m3);
    }

    #[test]
    fn identity() {
        let m4 = matrix_4x4![
            20, 22, 50, 48;
            44, 54, 114, 108;
            40, 58, 110, 102;
            16, 26, 46, 42];

        assert_eq!(m4.clone() * Matrix4x4::identity(), m4);
    }

    #[test]
    fn transpose() {
        let m = matrix_4x4![
            0, 9, 3, 0;
            9, 8, 0, 8;
            1, 8, 5, 3;
            0, 0, 4, 8];

        let t = matrix_4x4![
            0, 9, 1, 0;
            9, 8, 8, 0;
            3, 0, 5, 4;
            0, 8, 3, 8];

        assert_eq!(m.transpose(), t);
    }

    #[test]
    fn submatrix() {
        assert_eq!(
            matrix_4x4![
                -6, 1,  1, 6;
                -8, 5,  8, 6;
                -1, 0,  8, 6;
                -7, 1, -1, 1]
            .remove_indexes(2, 1),
            matrix_3x3![
                -6,  1, 6;
                -8,  8, 6;
                -7, -1, 1]
        )
    }

    // #[test]
    // fn determinant() {
    //     assert!(approx(matrix![1, 5; -3, 2].det(), 17.0));
    //     assert!(approx(matrix![1, 2, 6; -5, 8, -4; 2, 6, 4].det(), -196.0));
    //     assert!(approx(
    //         matrix![
    //             -2, -8,  3,  5;
    //             -3,  1,  7,  3;
    //              1,  2, -9,  6;
    //             -6,  7,  7, -9]
    //         .det(),
    //         -4071.0
    //     ));
    //
    //     assert!(approx(
    //         matrix![
    //             6, 4, 4, 4;
    //             5, 5, 7, 6;
    //             4, -9, 3, -7;
    //             9, 1, 7, -6]
    //         .det(),
    //         -2120.0
    //     ));
    // }
    // #[test]
    // fn minor_cofactor() {
    //     let m = matrix![3, 5, 0; 2, -1, -7; 6, -1, 5];
    //     assert!(approx(m.minor(0, 0), -12.0));
    //     assert!(approx(m.cofactor(0, 0), -12.0));
    //     assert!(approx(m.minor(1, 0), 25.0));
    //     assert!(approx(m.cofactor(1, 0), -25.0));
    //
    //     let m = matrix![
    //             -2, -8,  3,  5;
    //             -3,  1,  7,  3;
    //              1,  2, -9,  6;
    //             -6,  7,  7, -9];
    //
    //     assert!(approx(m.cofactor(0, 0), 690.0));
    //     assert!(approx(m.cofactor(0, 1), 447.0));
    //     assert!(approx(m.cofactor(0, 2), 210.0));
    //     assert!(approx(m.cofactor(0, 3), 51.0));
    //     assert!(approx(m.cofactor(1, 0), -253.0));
    //     assert!(approx(m.cofactor(1, 1), -394.0));
    //
    //     let m = matrix![
    //             -5, 2, 6, -8;
    //             1, -5, 1, 8;
    //             7, 7, -6, -7;
    //             1, -3, 7, 4
    //     ];
    //
    //     assert!(approx(m.cofactor(0, 0), 116.0));
    //     assert!(approx(m.cofactor(0, 1), -430.0));
    //     assert!(approx(m.cofactor(0, 2), -42.0));
    //     assert!(approx(m.cofactor(0, 3), -278.0));
    //
    //     assert!(approx(m.cofactor(1, 0), 240.0));
    //     assert!(approx(m.cofactor(1, 1), -775.0));
    //     assert!(approx(m.cofactor(1, 2), -119.0));
    //     assert!(approx(m.cofactor(1, 3), -433.0));
    //
    //     assert!(approx(m.cofactor(2, 0), 128.0));
    //     assert!(approx(m.cofactor(2, 1), -236.0));
    //     assert!(approx(m.cofactor(2, 2), -28.0));
    //     assert!(approx(m.cofactor(2, 3), -160.0));
    // }
    //
    // #[test]
    // fn inverse() {
    //     let m = matrix![1, 3; -3, 12];
    //     assert_eq!(
    //         m.invert().unwrap(),
    //         matrix![4.0/7.0, -1.0/7.0; 1.0/7.0, 1.0/21.0]
    //     );
    //
    //     let m = matrix![
    //         8, -5, 9, 2;
    //         7, 5, 6, 1;
    //         -6, 0, 9, 6;
    //         -3, 0, -9, -4
    //     ];
    //
    //     let acc = matrix![
    //         -0.15385, -0.15385, -0.28205, -0.53846;
    //         -0.07692, 0.12308, 0.02564, 0.03077;
    //         0.35897, 0.35897, 0.43590, 0.92308;
    //         -0.69231, -0.69231, -0.76923, -1.92308
    //     ];
    //
    //     assert_eq!(acc, m.invert().unwrap());
    // }
}
