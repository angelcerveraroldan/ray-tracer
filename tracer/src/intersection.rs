use crate::shapes::Shapes;

#[derive(Debug)]
pub struct Intersection<'a> {
    pub time: f64,
    pub shape: &'a Shapes,
}

impl<'a> Intersection<'a> {
    pub fn new(time: f64, shape: &'a Shapes) -> Self {
        Self { time, shape }
    }
}

pub type Intersections<'a> = Vec<Intersection<'a>>;
