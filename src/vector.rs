use std::ops::Sub;

pub struct Vector {
    x: f64,
    y: f64,
}

pub type Point = Vector;

impl Sub for Point {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}