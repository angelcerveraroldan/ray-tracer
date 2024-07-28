use std::ops::Index;

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

pub struct Intersections<'a> {
    intersections: Vec<Intersection<'a>>,
}

impl<'a> Intersections<'a> {
    pub fn new(intersections: Vec<Intersection<'a>>) -> Self {
        Self { intersections }
    }
}

impl<'a> Index<usize> for Intersections<'a> {
    type Output = Intersection<'a>;
    fn index(&self, index: usize) -> &Self::Output {
        self.intersections.index(index)
    }
}
