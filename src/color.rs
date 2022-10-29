
pub mod common;
pub mod gamma;
pub mod pixelformat;

pub struct Color {
    r: f64,
    g: f64,
    b: f64,
    a: f64
}

impl Color {
    pub fn get_r(&self) -> f64 {
        self.r
    }

    pub fn set_r(&mut self, x:f64) -> & mut Self {
        self.r = x;
        self
    }
}