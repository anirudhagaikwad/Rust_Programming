use std::io::{self, Read};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::VecDeque;
use std::sync::mpsc::{self, Receiver, Sender};
use rand::Rng;

// Direction enum to represent the snake's current direction
#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Point struct to represent a position on the game grid
#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

// Snake struct to represent the snake's state
struct Snake {
    body: VecDeque<Point>,
    direction: Direction,
    growing: bool,
}

// Food struct to represent the food's position
struct Food {
    position: Point,
}

// Game struct to represent the overall game state
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

    fn draw(&self) {
        print!("{}[2J", 27 as char); // Clear the screen
        for y in 0..self.height {
            for x in 0..self.width {
                // Draw the border
                if x == 0 || x == self.width - 1 || y == 0 || y == self.height - 1 {
                    print!("#");
                } else if self.snake.body.iter().any(|segment| segment.x == x && segment.y == y) {
                    print!("O");
                } else if self.food.position.x == x && self.food.position.y == y {
                    print!("X");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
        println!("Score: {}", self.score);
    }
}

fn read_input(tx: Sender<Direction>) {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = [0; 1];

    loop {
        handle.read_exact(&mut buf).unwrap();
        let input = char::from(buf[0]);

        let direction = match input {
            'w' => Some(Direction::Up),
            's' => Some(Direction::Down),
            'a' => Some(Direction::Left),
            'd' => Some(Direction::Right),
            _ => None,
        };

        if let Some(d) = direction {
            tx.send(d).unwrap();
        }
    }
}

fn main() {
    let (tx, rx): (Sender<Direction>, Receiver<Direction>) = mpsc::channel();
    let game_width = 22;  // Increased width for border
    let game_height = 22; // Increased height for border
    let mut game = Game::new(game_width, game_height);

    thread::spawn(move || read_input(tx));

    let mut last_update = Instant::now();

    loop {
        if game.game_over {
            println!("Game Over! Final Score: {}", game.score);
            break;
        }

        if last_update.elapsed() >= Duration::from_millis(300) {  // Increase the delay to slow down the game
            if let Ok(direction) = rx.try_recv() {
                game.change_direction(direction);
            }

            game.update();
            game.draw();

            last_update = Instant::now();
        }
    }
}