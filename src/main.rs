extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

mod apple;
mod input;
mod snake;
mod vector;

use apple::Apple;
use glutin_window::GlutinWindow as Window;
use input::Press;

use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use snake::Snake;

pub const SPEED: f64 = 1.0;
pub const BOARD_SIZE: u32 = 200;
pub const TILE_SIZE: u32 = 20;
pub const UPDATE_TIME: f64 = 0.1;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    snake: Snake,
    apple: Apple,
    current_press: Press,
    time: f64,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw_begin(args.viewport());

        clear(GREEN, &mut self.gl);
        self.snake.draw(&mut self.gl, args);
        self.apple.draw(&mut self.gl, args);

        self.gl.draw_end();
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.time += args.dt;
        self.snake.update_direction(&self.current_press);

        if self.snake.apple_collision(&mut self.apple) {
            self.apple.eat();
            self.snake.grow();
        }

        if self.time > UPDATE_TIME {
            self.time = 0.0;
            self.snake.update();
        }
    }

    fn input(&mut self, button: &Button) {
        match button {
            Button::Keyboard(Key::A) => self.current_press = Press::Left,
            Button::Keyboard(Key::S) => self.current_press = Press::Down,
            Button::Keyboard(Key::W) => self.current_press = Press::Up,
            Button::Keyboard(Key::D) => self.current_press = Press::Right,
            _ => {
                println!("something else");
            }
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("moving square", [BOARD_SIZE, BOARD_SIZE])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        snake: Snake::new(),
        apple: Apple::new(),
        current_press: Press::Right,
        time: 0.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(inp) = e.press_args() {
            app.input(&inp);
        }
    }
}
