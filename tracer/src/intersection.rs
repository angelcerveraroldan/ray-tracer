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

    pub fn hit(&self) -> Option<&Intersection<'a>> {
        let mut f = None;
        let mut min_time = f64::INFINITY;
        for inter in &self.intersections {
            let time = inter.time;
            if time < min_time && time > 0.0 {
                min_time = time;
                f = Some(inter);
            }
        }
        f
    }
}

impl<'a> Index<usize> for Intersections<'a> {
    type Output = Intersection<'a>;
    fn index(&self, index: usize) -> &Self::Output {
        self.intersections.index(index)
    }
}
