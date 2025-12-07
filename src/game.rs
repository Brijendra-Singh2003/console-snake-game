use crossterm::event::{self, Event, KeyCode};
use rand::Rng;
use std::{
    collections::VecDeque, io::{self, Write}, time::{Duration, Instant}
};
use tokio::time::sleep;

use super::utils;

struct Coordinate {
    x: usize,
    y: usize,
}

pub struct Game {
    height: usize,
    width: usize,
    print_interval: Duration,
    last_print_time: Instant,
    board: Vec<Vec<u8>>,
    snake: VecDeque<Coordinate>,
    direction: (i32, i32),
    game_over: bool,
}

const EMPTY: u8 = 0;
const SNAKE: u8 = 8;
const FOOD: u8 = 2;

impl Game {
    pub fn new(height: usize, width: usize) -> Self {
        return Self {
            height: height,
            width: width,
            print_interval: Duration::from_millis(1000),
            last_print_time: Instant::now(),
            board: Vec::new(),
            snake: VecDeque::new(),
            direction: (1, 0),
            game_over: false,
        };
    }

    pub fn set_target_fps(&mut self, fps: u64) {
        let frame_duration_ms = 1000 / fps;
        self.print_interval = Duration::from_millis(frame_duration_ms);
    }

    fn reset(&mut self) {
        self.board = vec![vec![0; self.width]; self.height];
        self.snake = VecDeque::new();
        self.direction = (1, 0);
        self.game_over = false;

        let start_pos = Coordinate {
            x: 0,
            y: self.height / 2,
        };

        self.board[start_pos.y][start_pos.x] = SNAKE;
        self.snake.push_front(start_pos);
        self.spawn_food();
        utils::clear_screen();
    }

    pub async fn start(&mut self) {
        self.reset();

        loop {
            if event::poll(Duration::from_millis(1)).unwrap() {
                if let Event::Key(key_event) = event::read().unwrap() {
                    if key_event.code == KeyCode::Char('r') {
                        self.reset();
                    }

                    let next_direction = match key_event.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Up => (0, -1),
                        KeyCode::Down => (0, 1),
                        KeyCode::Left => (-1, 0),
                        KeyCode::Right => (1, 0),
                        _ => self.direction,
                    };

                    if next_direction.0 + self.direction.0 != 0 || next_direction.1 + self.direction.1 != 0 {
                        self.direction = next_direction;
                    }
                }
            }

            self.draw().await;
        }
    }

    fn update(&mut self) {
        let head = self.snake.front().unwrap();
        let tail = self.snake.back().unwrap();

        let next_head = Coordinate {
            x: (head.x as i32 + self.direction.0) as usize,
            y: (head.y as i32 + self.direction.1) as usize,
        };

        self.game_over = next_head.x >= self.width
            || next_head.y >= self.height
            || self.board[next_head.y][next_head.x] == SNAKE;

        if self.game_over {
            return;
        }

        let consuming_food = self.board[next_head.y][next_head.x] == FOOD;
        if !consuming_food {
            self.board[tail.y][tail.x] = EMPTY;
            self.snake.pop_back();
        }

        self.board[next_head.y][next_head.x] = SNAKE;
        self.snake.push_front(next_head);

        if consuming_food {
            self.spawn_food();
        }
    }

    async fn draw(&mut self) {
        let curr_time = Instant::now();

        if curr_time - self.last_print_time >= self.print_interval {
            self.last_print_time = curr_time;

            if self.game_over {
                utils::clear_screen();
                println!("Game Over!\nPress 'r' to restart or 'q' to quit.");
                return;
            }

            self.update();

            let mut s: String = String::from("\x1B[H");
            for row in &self.board {
                for cell in row {
                    let val;

                    if cell == &EMPTY {
                        val = "-  ";
                    } else if cell == &FOOD {
                        val = ":p ";
                    } else {
                        val = "@  ";
                    };

                    s.push_str(val);
                }
                s.push_str("\r\n");
            }
            println!("{}Press 'r' to restart or 'q' to quit.", s);

            io::stdout().flush().unwrap();
        }

        sleep(Duration::from_millis(1)).await;
    }

    fn spawn_food(&mut self) {
        let mut rng = rand::thread_rng();

        loop {
            let x = rng.gen::<usize>() % self.width;
            let y = rng.gen::<usize>() % self.height;

            if self.board[y][x] != 0 {
                continue;
            }

            self.board[y][x] = FOOD;
            break;
        }
    }
}
