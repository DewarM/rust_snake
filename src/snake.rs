use input::Press;

pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn add(&mut self, vec: &Vector) -> &mut Self {
        self.x += vec.x;
        self.y += vec.y;
        self
    }
}

impl Snake {
    pub fn new() -> Self {
        let position = Vector { x: 0.0, y: 0.0 };
        let velocity = Vector { x: 0.1, y: 0.0 };
        Snake {
            position: position,
            velocity: velocity,
        }
    }
    pub fn update(&mut self, press: &mut Press) {
        match press {
            Press::Left => self.velocity = Vector { x: -0.1, y: 0.0 },
            Press::Right => self.velocity = Vector { x: 0.1, y: 0.0 },
            Press::Up => self.velocity = Vector { x: 0.0, y: -0.1 },
            Press::Down => self.velocity = Vector { x: 0.0, y: 0.1 },
        }
        self.position.add(&self.velocity);
    }
}

pub struct Snake {
    pub position: Vector,
    pub velocity: Vector,
}
