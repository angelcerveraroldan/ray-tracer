pub mod commons;

pub mod square2;
pub mod square3;
pub mod square4;

use std::{
    ops::{Index, IndexMut, Mul},
    usize,
};

#[derive(Debug, Clone)]
/// More generic struct for a square matrix
///
/// For this use case, we will assume that S = 2, 3, or 4
/// as larger matrices will not be used in the ray tracer.
/// This is *purposely* not a square matrix of any size, as
/// a lot of the methods implemented would need a lot of
/// optimizing for a larger matrix
pub struct SquareMatrix<const S: usize> {
    pub data: Vec<Vec<f64>>,
}

impl<const S: usize> PartialEq for SquareMatrix<S> {
    fn eq(&self, other: &Self) -> bool {
        for row_index in 0..S {
            for col_index in 0..S {
                let a = self[(row_index, col_index)];
                let b = other[(row_index, col_index)];

                if !crate::approx::approx(a, b) {
                    return false;
                }
            }
        }

        true
    }
}

impl<const S: usize> Default for SquareMatrix<S> {
    fn default() -> Self {
        let row = vec![0.0; S];
        let data = vec![row; S];
        Self { data }
    }
}

impl<const S: usize> From<Vec<Vec<f64>>> for SquareMatrix<S> {
    fn from(data: Vec<Vec<f64>>) -> Self {
        Self { data }
    }
}

impl<const S: usize> From<[[f64; S]; S]> for SquareMatrix<S> {
    fn from(data: [[f64; S]; S]) -> Self {
        Self::new(data)
    }
}

impl<const S: usize> Index<usize> for SquareMatrix<S> {
    type Output = Vec<f64>;
    fn index(&self, index: usize) -> &Self::Output {
        self.data.index(index)
    }
}

impl<const S: usize> Index<(usize, usize)> for SquareMatrix<S> {
    type Output = f64;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row][col]
    }
}

impl<const S: usize> IndexMut<usize> for SquareMatrix<S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.data.index_mut(index)
    }
}

impl<const S: usize> IndexMut<(usize, usize)> for SquareMatrix<S> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        self.data.index_mut(row).index_mut(col)
    }
}

impl<const S: usize> SquareMatrix<S> {
    pub fn size() -> (usize, usize) {
        (S, S)
    }

    pub fn new(data: [[f64; S]; S]) -> SquareMatrix<S> {
        let data = data.iter().map(|row| Vec::from(row)).collect::<Vec<_>>();
        Self { data }
    }

    pub fn get_row(&self, id: usize) -> Vec<f64> {
        self.data.index(id).clone()
    }

    pub fn get_col(&self, id: usize) -> Vec<f64> {
        self.data.iter().map(|row| row[id]).collect()
    }

    pub fn map_elements<F>(&self, f: F) -> Self
    where
        F: Fn(&f64) -> f64,
    {
        SquareMatrix::<S>::from(
            self.data
                .iter()
                .map(|row| row.iter().map(|x| f(&x)).collect::<Vec<f64>>())
                .collect::<Vec<Vec<f64>>>(),
        )
    }

    pub fn mut_map_elements<F>(&mut self, f: F)
    where
        F: Fn(&f64) -> f64,
    {
        for i in 0..S {
            for j in 0..S {
                self.data[i][j] = f(&self.data[i][j]);
            }
        }
    }

    pub fn identity() -> Self {
        let mut m = Self::default();
        for i in 0..S {
            m[i][i] = 1.0
        }
        m
    }

    pub fn transpose(&self) -> Self {
        let mut m = Self::default();
        for i in 0..S {
            for j in 0..S {
                m.data[i][j] = self.data[j][i]
            }
        }
        m
    }

    #[inline(always)]
    pub fn mutate_to(&mut self, pos: (usize, usize), to: f64) {
        self[pos] = to;
    }
}
