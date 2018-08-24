use input::Press;
use vector::Vector;
use graphics::rectangle;
use opengl_graphics::{GlGraphics};
use piston::input::*;
use TILE_SIZE;

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

    pub fn head(&mut self) -> Vector {
        self.tail.first().unwrap().clone()
    }

    pub fn grow(&mut self) {
        let tail = self.tail.last().unwrap().clone();
        self.tail.push(tail);
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
    pub direction: Vector,
    pub tail: Vec<Vector>,
}
