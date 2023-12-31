use std::ops::Sub;

#[derive(Copy, Clone)]
pub struct Vector {
   pub x: f64,
   pub y: f64,
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