#[derive(Debug, PartialEq)]
pub struct Matrix {
    height: usize,
    width: usize,
    rows: Vec<Vec<f64>>,
}
// TODO
//  Multiply by tupl
//  Multiply matrices of different size
impl Matrix {
    /// Make a new square matrix
    pub fn new(size: usize, rows: Vec<Vec<f64>>) -> Self {
        Self {
            height: size,
            width: size,
            rows,
        }
    }

    pub fn identity(size: usize) -> Self {
        let mut rows = vec![vec![0.0; size]; size];
        for i in 0..size {
            rows[i][i] = 1.0;
        }

        Self {
            height: size,
            width: size,
            rows,
        }
    }

    /// # Panics
    ///
    /// Will panic if i is not in range
    pub fn get_row(&self, i: usize) -> Vec<f64> {
        self.rows[i].clone()
    }

    /// # Panics
    ///
    /// Will panic if i is not in range
    pub fn get_col(&self, i: usize) -> Vec<f64> {
        self.rows.iter().map(|row| row[i]).collect::<Vec<f64>>()
    }

    /// Try to multiply two matrices
    ///
    /// Will fail when matrices are not the same size
    pub fn times(&self, rhs: Matrix) -> Result<Matrix, String> {
        if self.height != rhs.height || self.width != rhs.width {
            return Err("Matrices are not the same size".to_string());
        }

        let rows = (0..self.height).map(|row_index| {
            let row = self.get_row(row_index);
            (0..self.width)
                .map(|col_index| {
                    row.iter()
                        .zip(rhs.get_col(col_index))
                        .map(|(a, b)| a * b)
                        .sum()
                })
                .collect()
        });

        Ok(Matrix::new(self.height, rows.collect()))
    }
}

#[cfg(test)]
mod matrix_test {
    use super::*;

    // TODO: Make a macro to make matrix easier...
    #[test]
    fn matrix_multiply() {
        let m1 = Matrix::new(
            4,
            vec![
                vec![1.0, 2.0, 3.0, 4.0],
                vec![1.0, 2.0, 3.0, 4.0],
                vec![1.0, 2.0, 3.0, 4.0],
                vec![1.0, 2.0, 3.0, 4.0],
            ],
        );

        let m2 = Matrix::new(
            4,
            vec![
                vec![1.0, 2.0, 3.0, 4.0],
                vec![1.0, 2.0, 3.0, 4.0],
                vec![1.0, 2.0, 3.0, 4.0],
                vec![1.0, 2.0, 3.0, 4.0],
            ],
        );

        let m3 = Matrix::new(
            4,
            vec![
                vec![10.0, 20.0, 30.0, 40.0],
                vec![10.0, 20.0, 30.0, 40.0],
                vec![10.0, 20.0, 30.0, 40.0],
                vec![10.0, 20.0, 30.0, 40.0],
            ],
        );

        assert_eq!(m1.times(m2).unwrap(), m3);

        let m1 = Matrix::new(
            4,
            vec![
                vec![1.0, 2.0, 3.0, 4.0],
                vec![5.0, 6.0, 7.0, 8.0],
                vec![9.0, 8.0, 7.0, 6.0],
                vec![5.0, 4.0, 3.0, 2.0],
            ],
        );

        let m2 = Matrix::new(
            4,
            vec![
                vec![-2.0, 1.0, 2.0, 3.0],
                vec![3.0, 2.0, 1.0, -1.0],
                vec![4.0, 3.0, 6.0, 5.0],
                vec![1.0, 2.0, 7.0, 8.0],
            ],
        );

        let m3 = Matrix::new(
            4,
            vec![
                vec![20.0, 22.0, 50.0, 48.0],
                vec![44.0, 54.0, 114.0, 108.0],
                vec![40.0, 58.0, 110.0, 102.0],
                vec![16.0, 26.0, 46.0, 42.0],
            ],
        );

        assert_eq!(m1.times(m2).unwrap(), m3);
    }

    #[test]
    fn identity() {
        let id2 = Matrix::identity(2);
        let m2x2 = Matrix::new(2, vec![vec![1.0, 0.0], vec![0.0, 1.0]]);
        assert_eq!(id2, m2x2);

        let id3 = Matrix::identity(3);
        let m3x3 = Matrix::new(
            3,
            vec![
                vec![1.0, 0.0, 0.0],
                vec![0.0, 1.0, 0.0],
                vec![0.0, 0.0, 1.0],
            ],
        );
        assert_eq!(id3, m3x3);
    }
}
