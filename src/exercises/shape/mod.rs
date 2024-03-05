#[derive(Debug, Clone)]
pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Rectangle {
        Rectangle {
            width: width,
            height: height,
        }
    }

    pub fn get_area(&self) -> f64 {
        self.width * self.height
    }

    pub fn scale(&mut self, scale: f64) -> () {
        self.width *= scale;
        self.height *= scale;
    }
}
