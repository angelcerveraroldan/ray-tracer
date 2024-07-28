use crate::shapes::Shapes;

pub struct Intersection<'a> {
    pub time: f64,
    pub shape: &'a Shapes,
}