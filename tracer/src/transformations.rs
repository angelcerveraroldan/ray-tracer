use std::ops::Mul;

use crate::{
    matrix::square4::Matrix4x4,
    point::{coord::Coord, vector::Vector},
    ray::Ray,
};

/*
*
* TODO:
*   - Reflection
*
* */

/// Structure used to genrate matrices for translations
/// and to build more complex matrices that will transform
/// objects in many ways
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TransformationMatrix {
    pub matrix: Matrix4x4,
}

impl Default for TransformationMatrix {
    fn default() -> Self {
        Self {
            matrix: Matrix4x4::identity(),
        }
    }
}

impl TransformationMatrix {
    pub fn identity() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Generate a 4x4 translation matrix
    fn translation(by: Coord) -> Matrix4x4 {
        let mut m = Matrix4x4::identity();
        m.mutate_to((0, 3), by.x);
        m.mutate_to((1, 3), by.y);
        m.mutate_to((2, 3), by.z);
        m
    }

    /// Add translation to the current matrix
    pub fn translate<C>(&mut self, by: C) -> &mut Self
    where
        Coord: From<C>,
    {
        self.matrix = Self::translation(Coord::from(by)) * self.matrix;
        self
    }

    fn scaling(by: Coord) -> Matrix4x4 {
        let mut id4x4 = Matrix4x4::identity();
        id4x4.mutate_to((0, 0), by.x);
        id4x4.mutate_to((1, 1), by.y);
        id4x4.mutate_to((2, 2), by.z);
        id4x4
    }

    /// Add scaling to the current matrix
    pub fn scale<C>(&mut self, by: C) -> &mut Self
    where
        Coord: From<C>,
    {
        self.matrix = Self::scaling(Coord::from(by)) * self.matrix;
        self
    }

    fn rotation_x(rads: f64) -> Matrix4x4 {
        let mut id4x4 = Matrix4x4::identity();

        id4x4.mutate_to((1, 1), f64::cos(rads));
        id4x4.mutate_to((1, 2), -f64::sin(rads));

        id4x4.mutate_to((2, 1), f64::sin(rads));
        id4x4.mutate_to((2, 2), f64::cos(rads));

        id4x4
    }

    /// Rotate around the y axis by some radians
    fn rotation_y(rads: f64) -> Matrix4x4 {
        let mut id4x4 = Matrix4x4::identity();

        id4x4.mutate_to((0, 0), f64::cos(rads));
        id4x4.mutate_to((0, 2), f64::sin(rads));

        id4x4.mutate_to((2, 0), -f64::sin(rads));
        id4x4.mutate_to((2, 2), f64::cos(rads));

        id4x4
    }

    /// Rotate around the z axis by some radians
    fn rotation_z(rads: f64) -> Matrix4x4 {
        let mut id4x4 = Matrix4x4::identity();

        id4x4.mutate_to((0, 0), f64::cos(rads));
        id4x4.mutate_to((0, 1), -f64::sin(rads));

        id4x4.mutate_to((1, 0), f64::sin(rads));
        id4x4.mutate_to((1, 1), f64::cos(rads));

        id4x4
    }

    pub fn rotate_x(&mut self, rads: f64) -> &mut Self {
        self.matrix = Self::rotation_x(rads) * self.matrix;
        self
    }

    pub fn rotate_y(&mut self, rads: f64) -> &mut Self {
        self.matrix = Self::rotation_y(rads) * self.matrix;
        self
    }

    pub fn rotate_z(&mut self, rads: f64) -> &mut Self {
        self.matrix = Self::rotation_z(rads) * self.matrix;
        self
    }

    /// Apply all of the translations built up to an object
    pub fn apply<A>(&self, object: &A) -> A
    where
        crate::matrix::square4::Matrix4x4: for<'a> Mul<&'a A, Output = A>,
    {
        self.matrix * object
    }
}

pub enum Axis {
    X,
    Y,
    Z,
}

/// Apply a *single* transformation to an object
///
/// Since transformations are just 4x4 matrices, the object must be able to be multiplied with a
/// 4x4 matrix
///
/// To apply many transformations, you can chain the functions in this trait. If you want to run
/// the same sequence of transformations on many objects, it is more effiecnt to generate one
/// matrix that will apply all of those transformations using the TransformationMatrix struct.
pub trait Transform
where
    Self: Sized,
    crate::matrix::square4::Matrix4x4: for<'a> Mul<&'a Self, Output = Self>,
{
    /// Tanslate (move) by a certain ammount. Note that vectors cannot be translated
    fn translate<T>(&self, by: T) -> Self
    where
        Coord: From<T>,
    {
        TransformationMatrix::translation(Coord::from(by)) * self
    }

    fn scale<T>(&self, by: T) -> Self
    where
        Coord: From<T>,
    {
        TransformationMatrix::scaling(Coord::from(by)) * self
    }

    /// Rotate point around some axis by some radians
    fn rotate(&self, around: Axis, by: f64) -> Self {
        let roatation_matrix = match around {
            Axis::X => TransformationMatrix::rotation_x(by),
            Axis::Y => TransformationMatrix::rotation_y(by),
            Axis::Z => TransformationMatrix::rotation_z(by),
        };

        roatation_matrix * self
    }
}

impl Transform for Ray {
    fn translate<T>(&self, by: T) -> Self
    where
        Coord: From<T>,
    {
        let new_origin = self.origin.translate(by);
        Self::new(new_origin, self.dir.clone())
    }
}

impl Transform for Coord {}

impl Transform for Vector {
    // Vectors cannot be translated
    fn translate<T>(&self, _: T) -> Self {
        self.clone()
    }
}

#[cfg(test)]
mod test_transformations {
    use std::f64::consts::PI;

    use super::{Axis, Transform, TransformationMatrix};
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

    #[test]
    fn test_rotationx() {
        let p = Coord::from((0, 1, 0));

        assert_eq!(
            Coord::from((0.0, 1.0 / (2_f64).sqrt(), 1.0 / (2_f64.sqrt()))),
            p.rotate(Axis::X, PI / 4.0)
        );

        assert_eq!(Coord::from((0.0, 0.0, 1.0)), p.rotate(Axis::X, PI / 2.0));
    }

    #[test]
    fn test_rotationy() {
        let p = Coord::from((0, 0, 1));
        assert_eq!(
            Coord::from((1.0 / (2_f64).sqrt(), 0.0, 1.0 / (2_f64.sqrt()))),
            p.rotate(Axis::Y, PI / 4.0)
        );
        assert_eq!(Coord::from((1.0, 0.0, 0.0)), p.rotate(Axis::Y, PI / 2.0));
    }

    #[test]
    fn test_rotationz() {
        let p = Coord::from((0, 1, 0));
        assert_eq!(
            Coord::from((-1.0 / (2_f64).sqrt(), 1.0 / (2_f64.sqrt()), 0.0)),
            p.rotate(Axis::Z, PI / 4.0)
        );
        assert_eq!(Coord::from((-1.0, 0.0, 0.0)), p.rotate(Axis::Z, PI / 2.0));
    }

    #[test]
    fn build_translations() {
        let p = Coord::from((0, 1, 0));
        let np = p.scale((2.0, 1.0, 2.0)).translate((0.0, 0.0, 1.0));
        let mut transformation_matrix = TransformationMatrix::identity();
        transformation_matrix
            .scale((2.0, 1.0, 2.0))
            .translate((0.0, 0.0, 1.0));
        let op = transformation_matrix.apply(&p);
        assert_eq!(op, np);
    }
}
