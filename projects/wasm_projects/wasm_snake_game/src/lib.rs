use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};
use js_sys::Math;
use std::rc::Rc;
use std::cell::RefCell;

// Constants
const SQUARE_SIZE: f64 = 20.0;
const WIDTH: u32 = 20;
const HEIGHT: u32 = 20;

// Direction enum to represent the snake's current direction
#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Point struct to represent a position on the game grid
#[derive(Clone)]
struct Point {
    x: u32,
    y: u32,
}

// Snake struct to represent the snake's state
struct Snake {
    body: Vec<Point>,
    direction: Direction,
}

impl Snake {
    fn new(start_x: u32, start_y: u32) -> Snake {
        let body = vec![Point { x: start_x, y: start_y }];
        Snake {
            body,
            direction: Direction::Right,
        }
    }

    fn head_position(&self) -> &Point {
        &self.body[0]
    }

    fn change_direction(&mut self, new_direction: Direction) {
        self.direction = new_direction;
    }

    fn move_forward(&mut self) {
        let head = self.head_position().clone();
        let new_head = match self.direction {
            Direction::Up => Point { x: head.x, y: head.y.wrapping_sub(1) },
            Direction::Down => Point { x: head.x, y: head.y.wrapping_add(1) },
            Direction::Left => Point { x: head.x.wrapping_sub(1), y: head.y },
            Direction::Right => Point { x: head.x.wrapping_add(1), y: head.y },
        };
        self.body.insert(0, new_head);
        self.body.pop();
    }

    fn grow(&mut self) {
        let tail = self.body.last().unwrap().clone();
        self.body.push(tail);
    }

    fn check_collision(&self, width: u32, height: u32) -> bool {
        let head = self.head_position();
        // Check collision with walls
        if head.x >= width || head.y >= height {
            return true;
        }
        // Check collision with itself
        for segment in self.body.iter().skip(1) {
            if head.x == segment.x && head.y == segment.y {
                return true;
            }
        }
        false
    }
}

// Food struct to represent the food's position
struct Food {
    position: Point,
}

impl Food {
    fn new(x: u32, y: u32) -> Food {
        Food {
            position: Point { x, y },
        }
    }
}

// Game struct to represent the overall game state
struct Game {
    snake: Snake,
    food: Food,
    width: u32,
    height: u32,
    context: CanvasRenderingContext2d,
}

impl Game {
    fn new(context: CanvasRenderingContext2d, width: u32, height: u32) -> Game {
        let snake = Snake::new(width / 2, height / 2);
        let food = Food::new(
            (Math::random() * width as f64) as u32,
            (Math::random() * height as f64) as u32,
        );
        Game {
            snake,
            food,
            width,
            height,
            context,
        }
    }

    fn draw(&self) {
        self.context.set_fill_style(&JsValue::from_str("black"));
        self.context.fill_rect(0.0, 0.0, (self.width as f64) * SQUARE_SIZE, (self.height as f64) * SQUARE_SIZE);

        self.context.set_fill_style(&JsValue::from_str("green"));
        for segment in &self.snake.body {
            self.context.fill_rect((segment.x as f64) * SQUARE_SIZE, (segment.y as f64) * SQUARE_SIZE, SQUARE_SIZE, SQUARE_SIZE);
        }

        self.context.set_fill_style(&JsValue::from_str("red"));
        self.context.fill_rect(
            (self.food.position.x as f64) * SQUARE_SIZE,
            (self.food.position.y as f64) * SQUARE_SIZE,
            SQUARE_SIZE,
            SQUARE_SIZE,
        );
    }

    fn update(&mut self) {
        self.snake.move_forward();

        // Check if the snake ate the food
        if self.snake.head_position().x == self.food.position.x && self.snake.head_position().y == self.food.position.y {
            self.snake.grow();
            self.food = Food::new(
                (Math::random() * self.width as f64) as u32,
                (Math::random() * self.height as f64) as u32,
            );
        }

        // Check for collisions
        if self.snake.check_collision(self.width, self.height) {
            web_sys::console::log_1(&"Game Over!".into());
        }
    }

    fn change_direction(&mut self, new_direction: Direction) {
        self.snake.change_direction(new_direction);
    }
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>()?;
    let context = canvas.get_context("2d")?.unwrap().dyn_into::<CanvasRenderingContext2d>()?;

    let width = WIDTH;
    let height = HEIGHT;
    let game = Rc::new(RefCell::new(Game::new(context, width, height)));

    // Handle keyboard events
    let game_clone = game.clone();
    let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        match event.key().as_str() {
            "ArrowUp" | "w" => game_clone.borrow_mut().change_direction(Direction::Up),
            "ArrowDown" | "s" => game_clone.borrow_mut().change_direction(Direction::Down),
            "ArrowLeft" | "a" => game_clone.borrow_mut().change_direction(Direction::Left),
            "ArrowRight" | "d" => game_clone.borrow_mut().change_direction(Direction::Right),
            _ => {},
        }
    }) as Box<dyn FnMut(_)>);
    window.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
    closure.forget();

    // Game loop
    let f = Rc::new(RefCell::new(None::<Option<Closure<dyn FnMut()>>>));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        game.borrow_mut().update();
        game.borrow_mut().draw();

        // Schedule the next frame
        window
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    }) as Box<dyn FnMut()>));

    window
        .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .unwrap();
    Ok(())
}