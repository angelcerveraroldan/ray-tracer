use crate::matrix::{square2, square3, square4};

macro_rules! sqmatrix_commons {
    ($($t:ty),*) => {
        $(impl $t {
            pub fn minor(&self, row: usize, col: usize) -> f64 {
                self.remove_indexes(row, col).det()
            }

            pub fn cofactor(&self, row: usize, col: usize) -> f64 {
                (if (row + col) % 2 == 1 { -1.0 } else { 1.0 }) * self.minor(row, col)
            }

            pub fn inverse(&self) -> Option<Self> {
                let (size, _) = Self::size();

                let det = self.det();
                if  det == 0.0 { return None; }
                let det = 1.0/det;

                let data = (0..size)
                   .map(|row_index| {
                        (0..size)
                            .map(|col_index| self.cofactor(row_index, col_index) * det)
                            .collect()
                    })
                    .collect::<Vec<Vec<f64>>>();

                Some(Self::from(data).transpose())
            }
       })*
    };
}

sqmatrix_commons!(square3::Matrix3x3, square4::Matrix4x4);
