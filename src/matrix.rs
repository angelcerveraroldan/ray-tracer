use core::panic;
use std::fmt::Debug;

use crate::approx::approx;

// FIME:
//     matrix[
//         1, 2;
//         3, 4;
//     ];
//
//     will give an error (not sqaure matrix)
//
macro_rules! matrix {
    ($($($element:expr),*;)*) => {
        crate::matrix::SquareMatrix::from(vec![$( vec![ $($element),* ] ), *])
    };
    ($($($element:expr),*);*) => {
        crate::matrix::SquareMatrix::from(vec![$( vec![ $($element),* ] ), *])
    };
}

// Can we always know the size at compile time ? If so, we can make this a bit nicer
#[derive(Debug)]
struct SquareMatrix {
    pub size: usize,
    pub data: Vec<Vec<f64>>,
}

impl PartialEq for SquareMatrix {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }

        for row_index in 0..self.size {
            let row_self = &self.data[row_index];
            let row_other = &other.data[row_index];
            for col_index in 0..self.size {
                let a = row_self[col_index];
                let b = row_other[col_index];
                if !approx(a, b) {
                    return false;
                }
            }
        }

        true
    }
}

// TODO: This may be better as TryInto
impl<A: Into<f64> + Copy + Debug> From<Vec<Vec<A>>> for SquareMatrix {
    fn from(data: Vec<Vec<A>>) -> Self {
        let height = data.len();
        let width = data[0].len();

        if height != width {
            panic!("Not a square matrix!");
        }

        for row in &data {
            if row.len() != width {
                panic!("Not all rows are of the same length!");
            }
        }

        SquareMatrix {
            size: height,
            data: data
                .iter()
                .map(|row| row.iter().map(|&elem| elem.into()).collect())
                .collect(),
        }
    }
}

impl SquareMatrix {
    pub fn new(size: usize, rows: Vec<Vec<f64>>) -> Self {
        Self { size, data: rows }
    }

    pub fn identity(size: usize) -> Self {
        let mut data = vec![vec![0.0; size]; size];
        for i in 0..size {
            data[i][i] = 1.0;
        }

        Self { size, data }
    }

    pub fn map_elements<Func>(&self, f: Func) -> SquareMatrix
    where
        Func: Fn(&f64) -> f64,
    {
        SquareMatrix::new(
            self.size,
            self.data
                .iter()
                .map(|row| row.iter().map(|element| f(element)).collect())
                .collect(),
        )
    }

    /// # Panics
    ///
    /// Will panic if i is not in range
    pub fn get_row(&self, i: usize) -> Vec<f64> {
        self.data[i].clone()
    }

    /// # Panics
    ///
    /// Will panic if i is not in range
    pub fn get_col(&self, i: usize) -> Vec<f64> {
        self.data.iter().map(|row| row[i]).collect::<Vec<f64>>()
    }

    /// Try to multiply two matrices
    ///
    /// Will fail when matrices are not the same size
    pub fn times(&self, rhs: SquareMatrix) -> Option<SquareMatrix> {
        if self.size != rhs.size {
            return None;
        }

        let rows = (0..self.size).map(|row_index| {
            let row = self.get_row(row_index);
            (0..self.size)
                .map(|col_index| {
                    row.iter()
                        .zip(rhs.get_col(col_index))
                        .map(|(a, b)| a * b)
                        .sum()
                })
                .collect()
        });

        Some(SquareMatrix::new(self.size, rows.collect()))
    }

    fn times_tuple(&self, tuple: Vec<f64>) -> Option<Vec<f64>> {
        if self.size != tuple.len() {
            return None;
        }

        Some(
            self.data
                .iter()
                .map(|row| row.iter().zip(&tuple).map(|(a, b)| a * b).sum())
                .collect(),
        )
    }

    pub fn transpose(&self) -> SquareMatrix {
        let columns = (0..self.size)
            .map(|col| self.get_col(col))
            .collect::<Vec<Vec<_>>>();
        SquareMatrix::new(self.size, columns)
    }

    pub fn times_vec(
        &self,
        vec: crate::points::vector::Vector,
    ) -> Option<crate::points::vector::Vector> {
        let new = self.times_tuple(vec![vec.x, vec.y, vec.z])?;
        Some(crate::points::vector::Vector::new(new[0], new[1], new[2]))
    }

    pub fn times_coord(
        &self,
        coord: crate::points::coord::Coord,
    ) -> Option<crate::points::coord::Coord> {
        let new = self.times_tuple(vec![coord.x, coord.y, coord.z])?;
        Some(crate::points::coord::Coord::new(new[0], new[1], new[2]))
    }

    pub fn remove_indexes(&self, row: usize, col: usize) -> SquareMatrix {
        let mut data = self.data.clone();
        data.remove(row);
        data.iter_mut().for_each(|row| {
            row.remove(col);
        });

        SquareMatrix::new(self.size - 1, data)
    }

    pub fn det(&self) -> f64 {
        match self.size {
            2 => self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0],
            _ => (0..self.size)
                .map(|index| self.data[0][index] * self.cofactor(0, index))
                .sum(),
        }
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.remove_indexes(row, col).det()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        (if (row + col) % 2 == 1 { -1.0 } else { 1.0 }) * self.minor(row, col)
    }

    fn inverse2x2(&self, det: f64) -> SquareMatrix {
        let l = 1.0 / det;
        matrix![self.data[1][1], -self.data[0][1]; -self.data[1][0], self.data[0][0]]
            .map_elements(|x| x * l)
    }

    pub fn invert(&self) -> Option<SquareMatrix> {
        let det = self.det();
        if det == 0.0 {
            return None;
        }

        if self.size == 2 {
            return self.inverse2x2(det).into();
        }

        let data = (0..self.size)
            .map(|row_index| {
                (0..self.size)
                    .map(|col_index| self.cofactor(row_index, col_index))
                    .collect()
            })
            .collect::<Vec<Vec<f64>>>();

        SquareMatrix::from(data)
            .transpose()
            .map_elements(|x| x / det)
            .into()
    }
}

#[cfg(test)]
mod matrix_test {
    use crate::approx::approx;

    use super::*;
    #[test]
    fn macro_test() {
        let m1 = matrix![
            1, 2, 3, 4;
            1, 2, 3, 4;
            1, 2, 3, 4;
            1, 2, 3, 4];

        let m2 = SquareMatrix::new(
            4,
            vec![
                vec![1.0, 2.0, 3.0, 4.0],
                vec![1.0, 2.0, 3.0, 4.0],
                vec![1.0, 2.0, 3.0, 4.0],
                vec![1.0, 2.0, 3.0, 4.0],
            ],
        );

        assert_eq!(m1, m2);

        let m1 = matrix![
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
            matrix![1, 2; 3, 4].map_elements(|&n| n + 1.0),
            matrix![2, 3; 4, 5]
        )
    }

    #[test]
    fn matrix_multiply() {
        let m1 = matrix![
            1, 2, 3, 4;
            1, 2, 3, 4;
            1, 2, 3, 4;
            1, 2, 3, 4];

        let m2 = matrix![
            1, 2, 3, 4;
            1, 2, 3, 4;
            1, 2, 3, 4;
            1, 2, 3, 4];

        let m3 = matrix![
            10, 20, 30, 40;
            10, 20, 30, 40;
            10, 20, 30, 40;
            10, 20, 30, 40];

        assert_eq!(m1.times(m2).unwrap(), m3);

        let m1 = matrix![
            1, 2, 3, 4;
            5, 6, 7, 8;
            9, 8, 7, 6;
            5, 4, 3, 2];

        let m2 = matrix![
            -2, 1, 2, 3;
            3, 2, 1, -1;
            4, 3, 6, 5;
            1, 2, 7, 8];

        let m3 = matrix![
            20, 22, 50, 48;
            44, 54, 114, 108;
            40, 58, 110, 102;
            16, 26, 46, 42];

        assert_eq!(m1.times(m2).unwrap(), m3);
    }

    #[test]
    fn identity() {
        assert_eq!(SquareMatrix::identity(2), matrix![1, 0; 0, 1]);
        assert_eq!(
            SquareMatrix::identity(3),
            matrix![1, 0, 0; 0, 1, 0; 0, 0, 1]
        );

        let m4 = matrix![
            20, 22, 50, 48;
            44, 54, 114, 108;
            40, 58, 110, 102;
            16, 26, 46, 42];

        assert_eq!(m4.times(SquareMatrix::identity(4)).unwrap(), m4);
    }

    #[test]
    fn transpose() {
        let m = matrix![
            0, 9, 3, 0;
            9, 8, 0, 8;
            1, 8, 5, 3;
            0, 0, 4, 8];

        let t = matrix![
            0, 9, 1, 0;
            9, 8, 8, 0;
            3, 0, 5, 4;
            0, 8, 3, 8];

        assert_eq!(m.transpose(), t);
    }

    #[test]
    fn submatrix() {
        let m = matrix![1, 5, 0; -3, 2, 7; 0, 6, -3];

        assert_eq!(m.remove_indexes(0, 0), matrix![2, 7; 6, -3]);
        assert_eq!(m.remove_indexes(0, 1), matrix![-3, 7; 0, -3]);
        assert_eq!(m.remove_indexes(2, 1), matrix![1, 0; -3, 7]);

        assert_eq!(
            matrix![
                -6, 1,  1, 6;
                -8, 5,  8, 6;
                -1, 0,  8, 6;
                -7, 1, -1, 1]
            .remove_indexes(2, 1),
            matrix![
                -6,  1, 6;
                -8,  8, 6;
                -7, -1, 1]
        )
    }

    #[test]
    fn determinant() {
        assert!(approx(matrix![1, 5; -3, 2].det(), 17.0));
        assert!(approx(matrix![1, 2, 6; -5, 8, -4; 2, 6, 4].det(), -196.0));
        assert!(approx(
            matrix![
                -2, -8,  3,  5;
                -3,  1,  7,  3;
                 1,  2, -9,  6;
                -6,  7,  7, -9]
            .det(),
            -4071.0
        ));

        assert!(approx(
            matrix![
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
        let m = matrix![3, 5, 0; 2, -1, -7; 6, -1, 5];
        assert!(approx(m.minor(0, 0), -12.0));
        assert!(approx(m.cofactor(0, 0), -12.0));
        assert!(approx(m.minor(1, 0), 25.0));
        assert!(approx(m.cofactor(1, 0), -25.0));

        let m = matrix![
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

        let m = matrix![
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
        let m = matrix![1, 3; -3, 12];
        assert_eq!(
            m.invert().unwrap(),
            matrix![4.0/7.0, -1.0/7.0; 1.0/7.0, 1.0/21.0]
        );

        let m = matrix![
            8, -5, 9, 2;
            7, 5, 6, 1;
            -6, 0, 9, 6;
            -3, 0, -9, -4
        ];

        let acc = matrix![
            -0.15385, -0.15385, -0.28205, -0.53846;
            -0.07692, 0.12308, 0.02564, 0.03077;
            0.35897, 0.35897, 0.43590, 0.92308;
            -0.69231, -0.69231, -0.76923, -1.92308
        ];

        assert_eq!(acc, m.invert().unwrap());
    }
}
