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

pub const BOARD_SIZE: u32 = 200;
pub const TILE_SIZE: u32 = 20;
pub const UPDATE_TIME: f64 = 0.1;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    snake: Snake,
    apple: Apple,
    time: f64,
    state: GameState,
}

enum GameState {
    Running,
    Over,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        self.gl.draw_begin(args.viewport());

        clear(BLACK, &mut self.gl);
        self.snake.draw(&mut self.gl, args);
        self.apple.draw(&mut self.gl, args);

        self.gl.draw_end();
    }

    fn update(&mut self, args: &UpdateArgs) {
        match self.state {
            GameState::Running => {
                self.time += args.dt;

                if self.snake.tail_collision() {
                    self.state = GameState::Over;
                }

                if self.snake.apple_collision(&mut self.apple) {
                    self.apple.eat();
                    self.snake.grow();
                }

                if self.time > UPDATE_TIME {
                    self.time = 0.0;
                    self.snake.update();
                }
            }
            GameState::Over => (),
        }
    }

    fn reset(&mut self) {
        self.snake = Snake::new();
        self.apple = Apple::new();
        self.time = 0.0;
        self.state = GameState::Running;
    }

    fn input(&mut self, button: &Button) {
        match button {
            Button::Keyboard(Key::A) => self.snake.update_direction(&Press::Left),
            Button::Keyboard(Key::S) => self.snake.update_direction(&Press::Down),
            Button::Keyboard(Key::W) => self.snake.update_direction(&Press::Up),
            Button::Keyboard(Key::D) => self.snake.update_direction(&Press::Right),
            Button::Keyboard(Key::R) => self.reset(),
            _ => (),
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Snake", [BOARD_SIZE, BOARD_SIZE])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        snake: Snake::new(),
        apple: Apple::new(),
        time: 0.0,
        state: GameState::Running,
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
