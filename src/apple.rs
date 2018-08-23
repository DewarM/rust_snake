use vector::Vector;
use rand::prelude::*;
use graphics::rectangle;
use TILE_SIZE;
use piston::input::RenderArgs;
use opengl_graphics::GlGraphics;

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