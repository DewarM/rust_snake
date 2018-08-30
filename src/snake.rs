use apple::Apple;
use graphics::rectangle;
use input::Press;
use opengl_graphics::GlGraphics;
use piston::input::*;
use vector::Vector;
use BOARD_SIZE;
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
        let direction = Vector { x: -1.0, y: 0.0 };
        let tail = vec![
            Vector { x: 1.0, y: 0.0 },
            Vector { x: 2.0, y: 0.0 },
            Vector { x: 3.0, y: 0.0 },
        ];

        Snake {
            last_press: Press::Left,
            direction: direction,
            tail: tail,
            input_processed: false,
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

    pub fn tail_collision(&mut self) -> bool {
        let head = self.head();
        self.tail
            .clone()
            .into_iter()
            .skip(1)
            .fold(false, |acc, tail_vec| acc || head == tail_vec)
    }

    pub fn update_direction(&mut self, press: &Press) {
        if self.input_processed {
            let new_direction = match press {
                Press::Left => Vector { x: -1.0, y: 0.0 },
                Press::Right => Vector { x: 1.0, y: 0.0 },
                Press::Up => Vector { x: 0.0, y: -1.0 },
                Press::Down => Vector { x: 0.0, y: 1.0 },
            };
            if *press != dissallowed_press(&self.last_press) {
                self.direction = new_direction;
                self.last_press = press.clone();
                self.input_processed = false;
            }
        }
    }

    pub fn update(&mut self) {
        let mut tail = self.tail.clone();
        let mut new_head = self.head().add(&self.direction);

        let max_length = BOARD_SIZE as f64 / TILE_SIZE as f64;

        if new_head.x >= max_length {
            new_head.x = 0.0;
        }

        if new_head.x < 0.0 {
            new_head.x = max_length - 1.0;
        }

        if new_head.y >= max_length {
            new_head.y = 0.0;
        }

        if new_head.y < 0.0 {
            new_head.y = max_length - 1.0;
        }

        tail.pop();
        self.tail = vec![new_head];
        self.tail.append(&mut tail);
        self.input_processed = true;
    }

    pub fn draw(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
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
                rectangle(GREEN, square, c.transform, gl);
            }
        });
    }
}

pub struct Snake {
    last_press: Press,
    input_processed: bool,
    pub direction: Vector,
    pub tail: Vec<Vector>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_has_correct_tail_length() {
        let mut snake = Snake::new();
        snake.update();
        assert_eq!(snake.tail.len(), 3);
    }

    #[test]
    fn update_has_correct_tail_vecs() {
        let mut snake = Snake::new();
        snake.update();
        let mut tail_iter = snake.tail.into_iter();
        assert_eq!(tail_iter.next().unwrap(), Vector { x: 0.0, y: 0.0 });
        assert_eq!(tail_iter.next().unwrap(), Vector { x: 1.0, y: 0.0 });
        assert_eq!(tail_iter.next().unwrap(), Vector { x: 2.0, y: 0.0 });
    }
}
