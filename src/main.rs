extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

pub const SPEED: f64 = 1.0;
pub const BOARD_SIZE: u32 = 200;
pub const TILE_SIZE: u32 = 20;
pub const UPDATE_TIME: f64 = 0.15;

struct Vector {
    x: f64,
    y: f64,
}

impl Vector {
    fn add(&mut self, vec: &Vector) -> &mut Self {
        self.x += vec.x;
        self.y += vec.y;
        self
    }
}

enum Press {
    Left,
    Right,
    Up,
    Down,
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    position: Vector,
    velocity: Vector,
    current_press: Press,
    time: f64,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let (x, y) = (
            self.position.x * TILE_SIZE as f64,
            self.position.y * TILE_SIZE as f64,
        );
        let square = rectangle::square(x, y, TILE_SIZE as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c.transform;

            // Draw a box
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.time += args.dt;
        if self.time > UPDATE_TIME {
            self.time = 0.0;
        }
        match self.current_press {
            Press::Left => {
                self.velocity = Vector {
                    x: -0.1 * SPEED,
                    y: 0.0,
                }
            }
            Press::Right => {
                self.velocity = Vector {
                    x: 0.1 * SPEED,
                    y: 0.0,
                }
            }
            Press::Up => {
                self.velocity = Vector {
                    x: 0.0,
                    y: -0.1 * SPEED,
                }
            }
            Press::Down => {
                self.velocity = Vector {
                    x: 0.0,
                    y: 0.1 * SPEED,
                }
            }
        }

        self.position.add(&self.velocity);
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
    let mut window: Window = WindowSettings::new(
        "moving square",
        [BOARD_SIZE * TILE_SIZE, BOARD_SIZE * TILE_SIZE],
    ).opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        position: Vector { x: 0.0, y: 0.0 },
        velocity: Vector {
            x: 0.1 * SPEED,
            y: 0.0,
        },
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
