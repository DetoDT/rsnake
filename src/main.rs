use rand::prelude::*;
pub use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{self, Point, Rect};
use std::time::Duration;

const DIM: i32 = 600;
const RS: u32 = 30;

#[derive(Debug)]
pub struct Board {
    board: [[i8; 20]; 20],
}

impl Board {
    pub fn new() -> Self {
        Board {
            board: [[0; 20]; 20], // Initialize 30x30 grid with zeros
        }
    }
    fn new_food(&mut self) -> (i8, i8) {
        let mut rng = rand::rng();
        loop {
            let new_x = rng.random_range(0..20);
            let new_y = rng.random_range(0..20);
            if self.board[new_x][new_y] == 0 {
                self.board[new_x][new_y] = 2;
                return (new_x as i8, new_y as i8);
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

fn food_point(i: i8, j: i8) -> (i32, i32) {
    let x = i as i32 * DIM / RS as i32;
    let y = j as i32 * DIM / RS as i32;

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
    let mut snake = Rect::new(300, 300, RS, RS);
    let mut i = 0;

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
        canvas.fill_rect(snake).unwrap();
        let p = food_point(x, y);
        let point = Point::new(p.0, p.1);
        let food = Rect::new(point.x(), point.y(), RS, RS);
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.fill_rect(food).unwrap();
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
