use core::panic;
use std::fmt::Debug;

// FIME:
//     matrix[
//         1, 2;
//         3, 4;
//     ];
//
//     will give an error (not sqaure matrix)
//
macro_rules! matrix {
    ($($($element:expr),*);*) => {
        SquareMatrix::from(vec![$( vec![ $($element),* ] ), *])
    };
}

// Can we always know the size at compile time ? If so, we can make this a bit nicer
#[derive(Debug, PartialEq)]
struct SquareMatrix {
    pub size: usize,
    pub data: Vec<Vec<f64>>,
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

    pub fn det(&self) -> f64 {
        match self.size {
            2 => self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0],
            _ => panic!("Noooo"),
        }
    }

    pub fn remove_indexes(&self, row: usize, col: usize) -> SquareMatrix {
        let mut data = self.data.clone();
        data.remove(row);
        data.iter_mut().for_each(|row| {
            row.remove(col);
        });

        SquareMatrix::new(self.size - 1, data)
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
        let id2 = SquareMatrix::identity(2);
        let m2x2 = matrix![
            1, 0;
            0, 1];

        assert_eq!(id2, m2x2);

        let id3 = SquareMatrix::identity(3);
        let m3x3 = matrix![
            1, 0, 0;
            0, 1, 0;
            0, 0, 1];

        assert_eq!(id3, m3x3);

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
        let m = matrix![
             1, 5,  0;
            -3, 2,  7;
             0,  6, -3];

        assert_eq!(
            m.remove_indexes(0, 0),
            matrix![
                2,  7; 
                6, -3],
        );

        assert_eq!(
            m.remove_indexes(0, 1),
            matrix![
                -3,  7; 
                 0, -3],
        );

        assert_eq!(
            m.remove_indexes(2, 1),
            matrix![
                 1,  0; 
                -3,  7],
        );

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
        assert!(approx(
            matrix![
                 1, 5;
                -3, 2]
            .det(),
            17.0
        ));
    }
}
