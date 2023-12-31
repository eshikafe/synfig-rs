
pub mod common;
pub mod gamma;
pub mod pixelformat;

pub struct Color {
   pub r: f64,
   pub  g: f64,
   pub  b: f64,
   pub  a: f64
}

impl Color {
    pub fn get_r(&self) -> f64 {
        self.r
    }

    pub fn set_r(&mut self, x:f64) -> &mut Self {
        self.r = x;
        self
    }
}