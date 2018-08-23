use vector::Vector;
use rand::prelude::*;

pub struct Apple {
    pub position: Vector
}

impl Apple {

    pub fn new() -> Self {
        Apple {
            position: Vector { x: 0.0, y: 0.0 }
        }
    }

    pub fn eat(mut self) {
        let mut rng = thread_rng();
        let x = rng.gen_range(0.0, 10.0);
        let y = rng.gen_range(0.0, 10.0);
        self.position = Vector { x: x, y: y};
    }
}