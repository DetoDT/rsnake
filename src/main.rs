// TODO: Represent snake on matrix
// TODO: Keyboard change direction
// TODO: Snake body grows and food reposition
// TODO: Snake body follows body part in front of him
// TODO: Lose conditions
// TODO: Win condition
// TODO: Score and personal best
// TODO: Graphical tweak
// TODO: Menu
// TODO: Refactor into multiple files
// TODO: Comment code

use rand::prelude::*;
pub use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{self, Point, Rect};
use std::time::Duration;

const DIM: i32 = 600;
const RS: usize = 30;
const NCELL: usize = DIM as usize / RS;

#[derive(Debug)]
pub struct Board {
    board: [[i8; DIM as usize / RS]; DIM as usize / RS],
    snake: Snake,
    food: Food,
}

impl Board {
    pub fn new() -> Self {
        let mut b = Board {
            board: [[0; DIM as usize / RS]; DIM as usize / RS], // Initialize 30x30 grid with zeros
            snake: Snake::new(),
            food: Food::new(),
        };
        b.board[b.snake.pos.x][b.snake.pos.y] = 1;
        b.board[b.food.pos.x][b.food.pos.y] = 3;
        return b;
    }
    fn new_food(&mut self) -> (i8, i8) {
        let mut rng = rand::rng();
        loop {
            let new_x = rng.random_range(0..(DIM as usize - RS) / RS);
            let new_y = rng.random_range(0..(DIM as usize - RS) / RS);
            if self.board[new_x][new_y] == 0 {
                self.board[new_x][new_y] = 2;
                return (new_x as i8, new_y as i8);
            }
        }
        //(0_i8, 0_i8)
    }
    fn new_food_debug(&mut self, x: i8, y: i8) -> (i8, i8) {
        loop {
            if self.board[x as usize][y as usize] == 0 {
                self.board[x as usize][y as usize] = 2;
                return (x as i8, y as i8);
            }
        }
        //(0_i8, 0_i8)
    }
    pub fn print(&self) {
        for row in &self.board {
            println!("{:?}", row);
        }
    }
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x_n: usize, y_n: usize) -> Position {
        Position { x: x_n, y: y_n }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Snake {
    pos: Position,
    direction: Direction,
    length: i32,
    body: Rect,
}

impl Snake {
    fn new() -> Self {
        Snake {
            pos: Position::new(NCELL / 2 - 1, NCELL / 2 - NCELL / 4),
            direction: Direction::Right,
            length: 1,
            body: Rect::new(0, 10 * RS as i32, RS as u32, RS as u32),
        }
    }

    fn get_direction(&self) -> Direction {
        self.direction
    }

    fn move_snake(&mut self) {
        match self.direction {
            Direction::Up => self.body.set_y(self.body.y() + RS as i32),
            Direction::Down => self.body.set_y(self.body.y() + RS as i32),
            Direction::Left => self.body.set_x(self.body.x() - RS as i32),
            Direction::Right => self.body.set_x(self.body.x() + RS as i32),
        }
    }
}

#[derive(Debug)]
struct Food {
    pos: Position,
}

impl Food {
    fn update() -> Food {
        let mut rng = rand::rng();
        let new_x = rng.random_range(0..(DIM as usize - RS) / RS);
        let new_y = rng.random_range(0..(DIM as usize - RS) / RS);
        Food {
            pos: Position::new(new_x, new_y),
        }
    }

    fn new() -> Food {
        Food {
            pos: Position::new(NCELL / 2 - 1, NCELL / 2 + NCELL / 4),
        }
    }
}

fn food_point(i: i8, j: i8) -> (i32, i32) {
    let x = i as i32 * RS as i32;
    let y = j as i32 * RS as i32;

    return (x, y);
}

fn main() {
    let mut board = Board::new();
    board.print();
    let (x, y) = board.new_food();
    println!("{x} {y}");
    board.print();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("RSnake", 600, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut snake_body = Rect::new(0, 10 * RS as i32, RS as u32, RS as u32);
    let mut snake = Snake::new();
    let mut i = 0;

    let mut flag: bool = true;

    canvas.present();
    'running: loop {
        i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        board.snake.move_snake();
        canvas.fill_rect(board.snake.body).unwrap();

        let p = food_point(board.food.pos.x as i8, board.food.pos.y as i8);
        if flag {
            println!("{} {}", p.1, p.0);
            flag = false;
        }
        let point = Point::new(p.1, p.0);
        let food = Rect::new(point.x(), point.y(), RS as u32, RS as u32);

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.fill_rect(food).unwrap();
        canvas.present();

        ::std::thread::sleep(Duration::new(1, 0));
    }
}
