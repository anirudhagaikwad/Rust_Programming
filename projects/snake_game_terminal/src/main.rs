use snake_game_terminal::{KeyboardImpl, DisplayImpl};
use snake_game_terminal::keyboard::Keyboard;
use snake_game_terminal::display::Display;
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use rand::Rng;
/*
cargo build --release
target/release/snake_game_terminal.exe  # On Windows
./target/release/snake_game_terminal    # On Unix-like systems
*/
#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

struct Snake {
    body: VecDeque<Point>,
    direction: Direction,
    growing: bool,
}

struct Food {
    position: Point,
}

struct Game {
    snake: Snake,
    food: Food,
    width: i32,
    height: i32,
    score: u32,
    game_over: bool,
}

impl Snake {
    fn new(start_x: i32, start_y: i32) -> Self {
        let mut body = VecDeque::new();
        body.push_back(Point { x: start_x, y: start_y });
        Snake {
            body,
            direction: Direction::Right,
            growing: false,
        }
    }

    fn move_forward(&mut self) {
        let mut new_head = *self.body.front().unwrap();

        match self.direction {
            Direction::Up => new_head.y -= 1,
            Direction::Down => new_head.y += 1,
            Direction::Left => new_head.x -= 1,
            Direction::Right => new_head.x += 1,
        }

        self.body.push_front(new_head);

        if self.growing {
            self.growing = false;
        } else {
            self.body.pop_back();
        }
    }

    fn grow(&mut self) {
        self.growing = true;
    }

    fn change_direction(&mut self, new_direction: Direction) {
        if (self.direction == Direction::Up && new_direction != Direction::Down)
            || (self.direction == Direction::Down && new_direction != Direction::Up)
            || (self.direction == Direction::Left && new_direction != Direction::Right)
            || (self.direction == Direction::Right && new_direction != Direction::Left)
        {
            self.direction = new_direction;
        }
    }

    fn head_position(&self) -> Point {
        *self.body.front().unwrap()
    }

    fn check_collision(&self, width: i32, height: i32) -> bool {
        let head = self.head_position();

        // Check for collision with borders
        if head.x <= 0 || head.y <= 0 || head.x >= width - 1 || head.y >= height - 1 {
            return true;
        }

        // Check for collision with itself
        for segment in self.body.iter().skip(1) {
            if head.x == segment.x && head.y == segment.y {
                return true;
            }
        }

        false
    }
}

impl Food {
    fn new(x: i32, y: i32) -> Self {
        Food {
            position: Point { x, y },
        }
    }
}

impl Game {
    fn new(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let snake = Snake::new(width / 2, height / 2);
        let food = Food::new(rng.gen_range(1..width-1), rng.gen_range(1..height-1));
        Game {
            snake,
            food,
            width,
            height,
            score: 0,
            game_over: false,
        }
    }

    fn update(&mut self) {
        self.snake.move_forward();

        if self.snake.head_position().x == self.food.position.x
            && self.snake.head_position().y == self.food.position.y
        {
            self.snake.grow();
            let mut rng = rand::thread_rng();
            self.food = Food::new(rng.gen_range(1..self.width-1), rng.gen_range(1..self.height-1));
            self.score += 1;
        }

        if self.snake.check_collision(self.width, self.height) {
            self.game_over = true;
        }
    }

    fn change_direction(&mut self, new_direction: Direction) {
        self.snake.change_direction(new_direction);
    }

    fn draw(&self, display: &impl Display) {
        let mut buffer = vec![vec![' '; self.width as usize]; self.height as usize];

        // Draw the border
        for x in 0..self.width {
            buffer[0][x as usize] = '#';
            buffer[(self.height - 1) as usize][x as usize] = '#';
        }
        for y in 0..self.height {
            buffer[y as usize][0] = '#';
            buffer[y as usize][(self.width - 1) as usize] = '#';
        }

        // Draw the snake
        for segment in &self.snake.body {
            buffer[segment.y as usize][segment.x as usize] = 'O';
        }

        // Draw the food
        buffer[self.food.position.y as usize][self.food.position.x as usize] = 'X';

        // Clear the screen and draw the buffer
        display.clear_screen();
        display.draw(&buffer);

        println!("Score: {}", self.score);
    }
}

fn main() {
    let keyboard = KeyboardImpl::new();
    let display = DisplayImpl::new();
    let game_width = 22;  // Increased width for border
    let game_height = 22; // Increased height for border
    let mut game = Game::new(game_width, game_height);

    let mut last_update = Instant::now();

    loop {
        if game.game_over {
            println!("Game Over! Final Score: {}", game.score);
            break;
        }

        if last_update.elapsed() >= Duration::from_millis(300) {  // Increase the delay to slow down the game
            if let Some(input) = keyboard.read_input() {
                let direction = match input {
                    'w' => Some(Direction::Up),
                    's' => Some(Direction::Down),
                    'a' => Some(Direction::Left),
                    'd' => Some(Direction::Right),
                    _ => None,
                };

                if let Some(direction) = direction {
                    game.change_direction(direction);
                }
            }

            game.update();
            game.draw(&display);

            last_update = Instant::now();
        }
    }
}