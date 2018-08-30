use graphics::rectangle;
use input::Press;
use apple::Apple;
use opengl_graphics::GlGraphics;
use piston::input::*;
use vector::Vector;
use TILE_SIZE;

fn dissallowed_press(press: &Press) -> Press {
    match press {
        Press::Left => Press::Right,
        Press::Right => Press::Left,
        Press::Up => Press::Down,
        Press::Down => Press::Up,
    }
}

impl Snake {
    pub fn new() -> Self {
        let direction = Vector { x: 1.0, y: 0.0 };
        let tail = vec![
            Vector { x: 1.0, y: 0.0 },
            Vector { x: 2.0, y: 0.0 },
            Vector { x: 3.0, y: 0.0 },
        ];

        Snake {
            last_press: Press::Right,
            direction: direction,
            tail: tail,
        }
    }

    pub fn head(&mut self) -> Vector {
        self.tail.first().unwrap().clone()
    }

    pub fn grow(&mut self) {
        let tail = self.tail.last().unwrap().clone();
        self.tail.push(tail);
    }

    pub fn apple_collision(&mut self, apple: &mut Apple) -> bool {
        let head = self.head();
        let apple_position = apple.position;
        head == apple_position
    }

    pub fn update_direction(&mut self, press: &Press) {
        let new_direction = match press {
            Press::Left => Vector { x: -1.0, y: 0.0 },
            Press::Right => Vector { x: 1.0, y: 0.0 },
            Press::Up => Vector { x: 0.0, y: -1.0 },
            Press::Down => Vector { x: 0.0, y: 1.0 },
        };
        if *press != dissallowed_press(&self.last_press) {
            self.direction = new_direction;
            self.last_press = press.clone();
        }
    }

    pub fn update(&mut self) {
        let mut tail = self.tail.clone();
        let head = tail.first().unwrap().add(&self.direction);
        tail.pop();
        self.tail = vec![head];
        self.tail.append(&mut tail);
    }

    pub fn draw(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        let squares = self.tail.clone().into_iter().map(|vec| {
            rectangle::square(
                vec.x * TILE_SIZE as f64,
                vec.y * TILE_SIZE as f64,
                TILE_SIZE as f64,
            )
        });
        gl.draw(args.viewport(), |c, gl| {
            // Draw boxes
            for square in squares {
                rectangle(RED, square, c.transform, gl);
            }
        });
    }
}

pub struct Snake {
    last_press: Press,
    pub direction: Vector,
    pub tail: Vec<Vector>,
}
