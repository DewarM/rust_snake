use graphics::rectangle;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use rand::prelude::*;
use vector::Vector;

use BOARD_SIZE;
use TILE_SIZE;

pub struct Apple {
    pub position: Vector,
}

impl Apple {
    pub fn new() -> Self {
        Apple {
            position: Vector::rand(),
        }
    }

    pub fn eat(&mut self) {
        self.position = Vector::rand();
    }

    pub fn draw(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        let apple = rectangle::square(
            self.position.x * TILE_SIZE as f64,
            self.position.y * TILE_SIZE as f64,
            TILE_SIZE as f64,
        );
        gl.draw(args.viewport(), |c, gl| {
            rectangle(RED, apple, c.transform, gl);
        });
    }
}
