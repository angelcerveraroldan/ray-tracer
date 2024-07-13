use std::ops::Mul;

use crate::point::{coord::Coord, vector::Vector};

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
    pub fn remove_indexes(&self, row: usize, col: usize) -> crate::matrix::square2::Matrix2x2 {
        let mut data = self.data.clone();
        data.remove(row);
        for r in data.iter_mut() {
            r.remove(col);
        }
        crate::matrix::square2::Matrix2x2::from(data)
    }

    // Ther may be a way to optimize this (diagonals method)
    pub fn det(&self) -> f64 {
        self[(0, 0)] * self.remove_indexes(0, 0).det()
            - self[(0, 1)] * self.remove_indexes(0, 1).det()
            + self[(0, 2)] * self.remove_indexes(0, 2).det()
    }
}

/**********************************************
    MATRIX OPERATIONS FOR VECTORS / COORDS
***********************************************/

impl Mul<Vector> for Matrix3x3 {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Self::Output {
        let x = self[(0, 0)] * rhs.x + self[(0, 1)] * rhs.y + self[(0, 2)] * rhs.z;
        let y = self[(1, 0)] * rhs.x + self[(1, 1)] * rhs.y + self[(1, 2)] * rhs.z;
        let z = self[(2, 0)] * rhs.x + self[(2, 1)] * rhs.y + self[(2, 2)] * rhs.z;
        Vector::new(x, y, z)
    }
}

impl Mul<&Vector> for Matrix3x3 {
    type Output = Vector;
    fn mul(self, rhs: &Vector) -> Self::Output {
        let x = self[(0, 0)] * rhs.x + self[(0, 1)] * rhs.y + self[(0, 2)] * rhs.z;
        let y = self[(1, 0)] * rhs.x + self[(1, 1)] * rhs.y + self[(1, 2)] * rhs.z;
        let z = self[(2, 0)] * rhs.x + self[(2, 1)] * rhs.y + self[(2, 2)] * rhs.z;
        Vector::new(x, y, z)
    }
}

impl Mul<Coord> for Matrix3x3 {
    type Output = Coord;
    fn mul(self, rhs: Coord) -> Self::Output {
        let x = self[(0, 0)] * rhs.x + self[(0, 1)] * rhs.y + self[(0, 2)] * rhs.z;
        let y = self[(1, 0)] * rhs.x + self[(1, 1)] * rhs.y + self[(1, 2)] * rhs.z;
        let z = self[(2, 0)] * rhs.x + self[(2, 1)] * rhs.y + self[(2, 2)] * rhs.z;
        Coord::new(x, y, z)
    }
}

impl Mul<&Coord> for Matrix3x3 {
    type Output = Coord;
    fn mul(self, rhs: &Coord) -> Self::Output {
        let x = self[(0, 0)] * rhs.x + self[(0, 1)] * rhs.y + self[(0, 2)] * rhs.z;
        let y = self[(1, 0)] * rhs.x + self[(1, 1)] * rhs.y + self[(1, 2)] * rhs.z;
        let z = self[(2, 0)] * rhs.x + self[(2, 1)] * rhs.y + self[(2, 2)] * rhs.z;
        Coord::new(x, y, z)
    }
}

#[cfg(test)]
mod matrix_3x3_test {
    use super::*;
    use crate::approx::approx;
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

        assert_eq!(i.remove_indexes(0, 0), matrix_2x2![2, 7; 6, -3]);
        assert_eq!(i.remove_indexes(0, 1), matrix_2x2![-3, 7; 0, -3]);
        assert_eq!(i.remove_indexes(2, 1), matrix_2x2![1, 0; -3, 7]);
    }

    #[test]
    fn determinat() {
        assert!(approx(
            matrix_3x3![
                 1, 2,  6; 
                -5, 8, -4; 
                 2, 6,  4]
            .det(),
            -196.0
        ));
    }

    #[test]
    fn minor_cofactor() {
        let m = matrix_3x3![3, 5, 0; 2, -1, -7; 6, -1, 5];
        assert!(approx(m.minor(0, 0), -12.0));
        assert!(approx(m.cofactor(0, 0), -12.0));
        assert!(approx(m.minor(1, 0), 25.0));
        assert!(approx(m.cofactor(1, 0), -25.0));
    }
}
