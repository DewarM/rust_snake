#[derive(Copy, Clone, Debug)]
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

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        self.x == other.x && self.y == other.y
    }
}
