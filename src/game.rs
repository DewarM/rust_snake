use snake::Snake;
use apple::Apple;
use score::Display;
use input::Press;
use piston::input::*;
use opengl_graphics::{GlGraphics};

use UPDATE_TIME;

pub enum GameState {
    Running,
    Over,
}

pub struct Game {
    snake: Snake,
    apple: Apple,
    time: f64,
    state: GameState,
    display: Display,
}

impl Game {

    pub fn new() -> Self {
        Game {
            snake: Snake::new(),
            apple: Apple::new(),
            time: 0.0,
            state: GameState::Running,
            display: Display { score: 0 },
        }
    }

    pub fn handle_input(&mut self, button: &Button) {
        match button {
            Button::Keyboard(Key::A) => self.snake.update_direction(&Press::Left),
            Button::Keyboard(Key::S) => self.snake.update_direction(&Press::Down),
            Button::Keyboard(Key::W) => self.snake.update_direction(&Press::Up),
            Button::Keyboard(Key::D) => self.snake.update_direction(&Press::Right),
            Button::Keyboard(Key::R) => self.reset(),
            _ => (),
        }
    }

    pub fn reset(&mut self) {
        self.display = Display {
            score: 0
        };
        self.snake = Snake::new();
        self.apple = Apple::new();
        self.time = 0.0;
        self.state = GameState::Running;
    }

    pub fn draw(&mut self, mut gl: &mut GlGraphics, args: &RenderArgs) {
        self.display.draw(&mut gl, args);
        self.snake.draw(&mut gl, args);
        self.apple.draw(&mut gl, args);
    }

    pub fn clear(&mut self, gl: &mut GlGraphics) {
        use graphics::*;
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        clear(BLACK, gl);
    }
    
    pub fn update(&mut self, args: &UpdateArgs) {
        match self.state {
            GameState::Running => {
                self.time += args.dt;

                if self.snake.tail_collision() {
                    self.state = GameState::Over;
                }

                if self.snake.apple_collision(&mut self.apple) {
                    self.apple.eat();
                    self.snake.grow();
                    self.display.increment();
                }

                if self.time > UPDATE_TIME {
                    self.time = 0.0;
                    self.snake.update();
                }
            }
            GameState::Over => (),
        }
    }

}