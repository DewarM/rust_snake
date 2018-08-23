#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn add(&self, vec: &Vector) -> Self {
        Vector {
            x: self.x + vec.x,
            y: self.y + vec.y,
        }
    }
}
