
use crate::vector::{Vector, Point};

#[derive(Copy, Clone)]
pub struct Segment
{
	p1: Point,
    p2: Point,
	t1: Vector,
    t2: Vector,
}

impl Segment {
    pub fn new(p1: Point, t1: Vector, p2:Point, t2: Vector) -> Self {
        Self{p1, p2, t1, t2}
    }
		
	pub fn new2(p1: Point, p2: Point) -> Self {
        Self{p1, p2, t1: p2-p1, t2: p2-p1}
    }
}