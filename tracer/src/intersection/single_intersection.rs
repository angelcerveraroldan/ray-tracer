use crate::shapes::Shapes;

#[derive(Debug)]
pub struct SingleIntersection<'a> {
    pub time: f64,
    pub shape: &'a Shapes,
}

impl<'a> SingleIntersection<'a> {
    pub fn new(time: f64, shape: &'a Shapes) -> Self {
        Self { time, shape }
    }
}
