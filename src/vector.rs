use rand::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn rand(start: u32, end: u32) -> Self {
        let mut rng = thread_rng();
        let x = rng.gen_range(start, end);
        let y = rng.gen_range(start, end);
        Vector {
            x: x as f64,
            y: y as f64,
        }
    }
    
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
