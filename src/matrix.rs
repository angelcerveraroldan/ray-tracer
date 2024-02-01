pub struct Matrix {
    height: usize,
    width: usize,
    rows: Vec<Vec<f64>>,
}

impl Matrix {
    /// Make a new square matrix
    pub fn new(size: usize, rows: Vec<Vec<f64>>) -> Self {
        Self {
            height: size,
            width: size,
            rows,
        }
    }

    pub fn get_row(&self, i: usize) -> Vec<f64> {
        self.rows[i].clone()
    }

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
            (0..self.width)
                .map(|col_index| {
                    self.get_row(row_index)
                        .iter()
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
    #[test]
    fn multiply() {
        // let m1 = Matrix::new(4, vec![])
    }
}
