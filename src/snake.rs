use input::Press;
use vector::Vector;

impl Snake {
    pub fn new() -> Self {
        let direction = Vector { x: 1.0, y: 0.0 };
        let tail = vec![
            Vector { x: 1.0, y: 0.0 },
            Vector { x: 2.0, y: 0.0 },
            Vector { x: 3.0, y: 0.0 },
        ];

        Snake {
            direction: direction,
            tail: tail,
        }
    }

    pub fn update_direction(&mut self, press: &mut Press) {
        match press {
            Press::Left => self.direction = Vector { x: -1.0, y: 0.0 },
            Press::Right => self.direction = Vector { x: 1.0, y: 0.0 },
            Press::Up => self.direction = Vector { x: 0.0, y: -1.0 },
            Press::Down => self.direction = Vector { x: 0.0, y: 1.0 },
        }
    }

    pub fn update(&mut self) {
        let mut tail = self.tail.clone();
        let head = tail.first().unwrap().add(&self.direction);
        tail.pop();
        self.tail = vec![head];
        self.tail.append(&mut tail);
    }
}

pub struct Snake {
    pub direction: Vector,
    pub tail: Vec<Vector>,
}
