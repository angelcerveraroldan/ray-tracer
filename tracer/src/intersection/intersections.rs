use crate::intersection::single_intersection::SingleIntersection;
use std::ops::Index;

/*
* TODO:
*   - The intersection tracker should be always kept in order
*     (maybe make it a binaryheap), this way we can find the min
*     element much faster
* */

pub struct IntersectionTracker<'a> {
    intersections: Vec<SingleIntersection<'a>>,
}

impl<'a> IntersectionTracker<'a> {
    pub fn new(intersections: Vec<SingleIntersection<'a>>) -> Self {
        Self { intersections }
    }

    pub fn push(&mut self, intersection: SingleIntersection<'a>) {
        self.intersections.push(intersection)
    }

    pub fn hit(&self) -> Option<&SingleIntersection<'a>> {
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

impl<'a> Index<usize> for IntersectionTracker<'a> {
    type Output = SingleIntersection<'a>;
    fn index(&self, index: usize) -> &Self::Output {
        self.intersections.index(index)
    }
}
