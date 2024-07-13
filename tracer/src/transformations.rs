use std::ops::Mul;

use crate::{
    matrix::square4::Matrix4x4,
    point::{coord::Coord, vector::Vector},
};

struct TransformationMatrix;

impl TransformationMatrix {
    fn translation(by: Coord) -> Matrix4x4 {
        let mut m = Matrix4x4::identity();
        m.mutate_to((0, 3), by.x);
        m.mutate_to((1, 3), by.y);
        m.mutate_to((2, 3), by.z);
        m
    }

    fn scaling(by: Coord) -> Matrix4x4 {
        let mut id4x4 = Matrix4x4::identity();
        id4x4.mutate_to((0, 0), by.x);
        id4x4.mutate_to((1, 1), by.y);
        id4x4.mutate_to((2, 2), by.z);
        id4x4
    }
}

pub trait Transform
where
    Self: Sized,
    crate::matrix::square4::Matrix4x4: for<'a> Mul<&'a Self, Output = Self>,
{
    fn translate(&self, by: Coord) -> Self {
        TransformationMatrix::translation(by) * self
    }

    fn scale(&self, by: Coord) -> Self {
        TransformationMatrix::scaling(by) * self
    }
}

impl Transform for Coord {}

impl Transform for Vector {
    // Vectors cannot be translated
    fn translate(&self, _: Coord) -> Self {
        self.clone()
    }
}

#[cfg(test)]
mod test_transformations {
    use super::{Transform, TransformationMatrix};
    use crate::{
        matrix::square4::Matrix4x4,
        point::{coord::Coord, vector::Vector},
    };

    #[test]
    fn translation_matrix() {
        let p = Coord::from((-3, 4, 5));
        let t = TransformationMatrix::translation(p);
        let m = Matrix4x4::from([
            [1., 0., 0., -3.],
            [0., 1., 0., 4.],
            [0., 0., 1., 5.],
            [0., 0., 0., 1.],
        ]);
        assert_eq!(t, m);
    }

    #[test]
    fn translate() {
        let p = Coord::from((-3, 4, 5));
        let exp = Coord::from((2, 1, 7));
        let acc = p.translate(Coord::from((5, -3, 2)));
        assert_eq!(exp, acc);

        let v = Vector::from((-3, 4, 5));
        let acc = v.translate(Coord::from((5, -3, 2)));
        assert_eq!(v, acc);
    }

    #[test]
    fn scale() {
        let p = Coord::from((-4, 6, 8));
        let v = Vector::from((-4, 6, 8));

        let exp_p = Coord::from((-8, 18, 32));
        let exp_v = Vector::from((-8, 18, 32));

        let acc_p = p.scale(Coord::from((2, 3, 4)));
        let acc_v = v.scale(Coord::from((2, 3, 4)));

        assert_eq!(exp_v, acc_v);
        assert_eq!(exp_p, acc_p);
    }
}
