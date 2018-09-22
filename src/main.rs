extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate find_folder;
extern crate piston;
extern crate rand;

mod apple;
mod input;
mod snake;
mod vector;
mod score;
mod game;

use glutin_window::GlutinWindow as Window;
use piston::event_loop::*;
use piston::input::*;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::window::WindowSettings;

use game::Game;

pub const BOARD_SIZE: u32 = 200;
pub const TILE_SIZE: u32 = 20;
pub const UPDATE_TIME: f64 = 0.1;

pub struct App {
    
    gl: GlGraphics, // OpenGL drawing backend.
    game: Game
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        self.gl.draw_begin(args.viewport());
        self.game.clear(&mut self.gl);
        self.game.draw(&mut self.gl, args);
        self.gl.draw_end();
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.game.update(args);
    }

    fn input(&mut self, button: &Button) {
        self.game.handle_input(button);
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
        game: Game::new()
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
