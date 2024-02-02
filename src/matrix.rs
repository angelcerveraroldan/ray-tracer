macro_rules! matrix {
    ($($($element:expr),*);*) => {
        Matrix::try_from(vec![$( vec![ $($element),* ] ), *])
    };
}

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

    pub fn transpose(&self) -> Matrix {
        let columns = (0..self.width).map(|col| self.get_col(col)).collect::<Vec<Vec<_>>>();
        Matrix::new(self.width, columns)
    }
}

impl<A: Into<f64> + Copy> TryFrom<Vec<Vec<A>>> for Matrix {
    type Error = String;

    fn try_from(value: Vec<Vec<A>>) -> Result<Self, Self::Error> {
        let height = value.len();
        let width = value[0].len();

        if width != height {
            return Err("Not a squae matrix".to_string());
        }

        let rows = value
            .iter()
            .map(|row| row.iter().map(|&x| x.into()).collect())
            .collect();

        Ok(Matrix::new(height, rows))
    }
}

#[cfg(test)]
mod matrix_test {
    use super::*;
    #[test]
    fn macro_test() {
        let m1 = matrix![
            1, 2, 3, 4;
            1, 2, 3, 4;
            1, 2, 3, 4;
            1, 2, 3, 4]
        .unwrap();

        let m2 = Matrix::new(
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

    // TODO: Make a macro to make matrix easier...
    #[test]
    fn matrix_multiply() {
        let m1 = matrix![
            1, 2, 3, 4;
            1, 2, 3, 4;
            1, 2, 3, 4;
            1, 2, 3, 4]
        .unwrap();

        let m2 = matrix![
            1, 2, 3, 4;
            1, 2, 3, 4;
            1, 2, 3, 4;
            1, 2, 3, 4]
        .unwrap();

        let m3 = matrix![
            10, 20, 30, 40;
            10, 20, 30, 40;
            10, 20, 30, 40;
            10, 20, 30, 40]
        .unwrap();

        assert_eq!(m1.times(m2).unwrap(), m3);

        let m1 = matrix![
            1, 2, 3, 4;
            5, 6, 7, 8;
            9, 8, 7, 6;
            5, 4, 3, 2]
        .unwrap();

        let m2 = matrix![
            -2, 1, 2, 3;
            3, 2, 1, -1;
            4, 3, 6, 5;
            1, 2, 7, 8]
        .unwrap();

        let m3 = matrix![
            20, 22, 50, 48;
            44, 54, 114, 108;
            40, 58, 110, 102;
            16, 26, 46, 42]
        .unwrap();

        assert_eq!(m1.times(m2).unwrap(), m3);
    }

    #[test]
    fn identity() {
        let id2 = Matrix::identity(2);
        let m2x2 = matrix![
            1, 0; 
            0, 1]
        .unwrap();

        assert_eq!(id2, m2x2);

        let id3 = Matrix::identity(3);
        let m3x3 = matrix![
            1, 0, 0;
            0, 1, 0;
            0, 0, 1]
        .unwrap();

        assert_eq!(id3, m3x3);

        let m4 = matrix![
            20, 22, 50, 48;
            44, 54, 114, 108;
            40, 58, 110, 102;
            16, 26, 46, 42]
        .unwrap();

        assert_eq!(m4.times(Matrix::identity(4)).unwrap(), m4);
    }

    #[test]
    fn transpose() {
        let m = matrix![
            0, 9, 3, 0; 
            9, 8, 0, 8;
            1, 8, 5, 3; 
            0, 0, 4, 8]
        .unwrap();

        let t = matrix![
            0, 9, 1, 0;
            9, 8, 8, 0; 
            3, 0, 5, 4; 
            0, 8, 3, 8]
        .unwrap();

        assert_eq!(m.transpose(), t);
    }
}
