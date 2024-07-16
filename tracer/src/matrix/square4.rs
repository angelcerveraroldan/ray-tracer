use square3::Matrix3x3;

use crate::point::{coord::Coord, vector::Vector};

use super::*;

pub type Matrix4x4 = SquareMatrix<4>;

#[macro_export]
macro_rules! matrix_4x4 {
    ($($($element:expr),*;)*) => {
        $crate::matrix::square4::Matrix4x4::from([$( [ $($element as f64),* ] ), *])
    };
    ($($($element:expr),*);*) => {
        $crate::matrix::square4::Matrix4x4::from([$( [ $($element as f64),* ] ), *])
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
    pub fn remove_indexes(&self, row: usize, col: usize) -> crate::matrix::square3::Matrix3x3 {
        let rows = (0..4).filter(|&x| x != row).enumerate();
        let cols = (0..4).filter(|&x| x != col).enumerate().collect::<Vec<_>>();
        let mut other = Matrix3x3::default();
        for (index_r, row_index) in rows {
            for (index_c, col_index) in &cols {
                other[(index_r, *index_c)] = self[(row_index, *col_index)];
            }
        }
        other
    }

    pub fn det(&self) -> f64 {
        self[(0, 0)] * self.remove_indexes(0, 0).det()
            - self[(0, 1)] * self.remove_indexes(0, 1).det()
            + self[(0, 2)] * self.remove_indexes(0, 2).det()
            - self[(0, 3)] * self.remove_indexes(0, 3).det()
    }
}

/**********************************************
    MATRIX OPERATIONS FOR VECTORS / COORDS
***********************************************/

impl Mul<Vector> for Matrix4x4 {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Self::Output {
        let x = self[(0, 0)] * rhs.x + self[(0, 1)] * rhs.y + self[(0, 2)] * rhs.z;
        let y = self[(1, 0)] * rhs.x + self[(1, 1)] * rhs.y + self[(1, 2)] * rhs.z;
        let z = self[(2, 0)] * rhs.x + self[(2, 1)] * rhs.y + self[(2, 2)] * rhs.z;
        Vector::new(x, y, z)
    }
}

impl Mul<&Vector> for Matrix4x4 {
    type Output = Vector;
    fn mul(self, rhs: &Vector) -> Self::Output {
        let x = self[(0, 0)] * rhs.x + self[(0, 1)] * rhs.y + self[(0, 2)] * rhs.z;
        let y = self[(1, 0)] * rhs.x + self[(1, 1)] * rhs.y + self[(1, 2)] * rhs.z;
        let z = self[(2, 0)] * rhs.x + self[(2, 1)] * rhs.y + self[(2, 2)] * rhs.z;
        Vector::new(x, y, z)
    }
}

impl Mul<Coord> for Matrix4x4 {
    type Output = Coord;
    fn mul(self, rhs: Coord) -> Self::Output {
        let x = self[(0, 0)] * rhs.x + self[(0, 1)] * rhs.y + self[(0, 2)] * rhs.z + self[(0, 3)];
        let y = self[(1, 0)] * rhs.x + self[(1, 1)] * rhs.y + self[(1, 2)] * rhs.z + self[(1, 3)];
        let z = self[(2, 0)] * rhs.x + self[(2, 1)] * rhs.y + self[(2, 2)] * rhs.z + self[(2, 3)];
        Coord::new(x, y, z)
    }
}

impl Mul<&Coord> for Matrix4x4 {
    type Output = Coord;
    fn mul(self, rhs: &Coord) -> Self::Output {
        let x = self[(0, 0)] * rhs.x + self[(0, 1)] * rhs.y + self[(0, 2)] * rhs.z + self[(0, 3)];
        let y = self[(1, 0)] * rhs.x + self[(1, 1)] * rhs.y + self[(1, 2)] * rhs.z + self[(1, 3)];
        let z = self[(2, 0)] * rhs.x + self[(2, 1)] * rhs.y + self[(2, 2)] * rhs.z + self[(2, 3)];
        Coord::new(x, y, z)
    }
}

#[cfg(test)]
mod matrix_4x4_test {
    use super::*;
    use crate::{approx::approx, matrix_3x3};

    #[test]
    fn macro_test() {
        let m1 = matrix_4x4![
            1, 2, 3, 4;
            1, 2, 3, 4;
            1, 2, 3, 4;
            1, 2, 3, 4];

        let m2 = Matrix4x4::from([
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

    #[test]
    fn determinant() {
        assert!(approx(
            matrix_4x4![
                -2, -8,  3,  5;
                -3,  1,  7,  3;
                 1,  2, -9,  6;
                -6,  7,  7, -9]
            .det(),
            -4071.0
        ));

        assert!(approx(
            matrix_4x4![
                6, 4, 4, 4;
                5, 5, 7, 6;
                4, -9, 3, -7;
                9, 1, 7, -6]
            .det(),
            -2120.0
        ));
    }

    #[test]
    fn minor_cofactor() {
        let m = matrix_4x4![
                -2, -8,  3,  5;
                -3,  1,  7,  3;
                 1,  2, -9,  6;
                -6,  7,  7, -9];

        assert!(approx(m.cofactor(0, 0), 690.0));
        assert!(approx(m.cofactor(0, 1), 447.0));
        assert!(approx(m.cofactor(0, 2), 210.0));
        assert!(approx(m.cofactor(0, 3), 51.0));
        assert!(approx(m.cofactor(1, 0), -253.0));
        assert!(approx(m.cofactor(1, 1), -394.0));

        let m = matrix_4x4![
                -5, 2, 6, -8;
                1, -5, 1, 8;
                7, 7, -6, -7;
                1, -3, 7, 4
        ];

        assert!(approx(m.cofactor(0, 0), 116.0));
        assert!(approx(m.cofactor(0, 1), -430.0));
        assert!(approx(m.cofactor(0, 2), -42.0));
        assert!(approx(m.cofactor(0, 3), -278.0));

        assert!(approx(m.cofactor(1, 0), 240.0));
        assert!(approx(m.cofactor(1, 1), -775.0));
        assert!(approx(m.cofactor(1, 2), -119.0));
        assert!(approx(m.cofactor(1, 3), -433.0));

        assert!(approx(m.cofactor(2, 0), 128.0));
        assert!(approx(m.cofactor(2, 1), -236.0));
        assert!(approx(m.cofactor(2, 2), -28.0));
        assert!(approx(m.cofactor(2, 3), -160.0));
    }

    #[test]
    fn inverse() {
        let m = matrix_4x4![
            8, -5, 9, 2;
            7, 5, 6, 1;
            -6, 0, 9, 6;
            -3, 0, -9, -4
        ];

        let acc = matrix_4x4![
            -0.15385, -0.15385, -0.28205, -0.53846;
            -0.07692, 0.12308, 0.02564, 0.03077;
            0.35897, 0.35897, 0.43590, 0.92308;
            -0.69231, -0.69231, -0.76923, -1.92308
        ];

        assert_eq!(Some(acc), m.inverse());
    }

    #[test]
    fn times_vector() {
        let v = Vector::from((1, 2, 3));
        let m = Matrix4x4::identity();

        assert_eq!(v.clone(), (m * v.clone()));

        let m = matrix_4x4!(
            1, 2, 0, 1;
            4, 3, 1, 0;
            1, 2, 0, 1;
            4, 3, 1, 0;
        );

        let expected = Vector::from((5, 13, 5));
        assert_eq!(expected, (m * v.clone()));
    }

    #[test]
    fn times_coord() {
        let c = Coord::from((1, 2, 3));
        let m = matrix_4x4!(
            1, 2, 0, 1;
            4, 3, 1, 0;
            1, 2, 0, 1;
            4, 3, 1, 0;
        );

        let expected = Coord::from((6, 13, 6));
        assert_eq!(expected, (m * c.clone()));
    }
}
