use crossterm::event::{self, Event, KeyCode};
use rand::Rng;
use std::{
    collections::VecDeque, io::{self, Write}, time::{Duration, Instant}
};
use tokio::time::sleep;

struct Coordinate {
    x: usize,
    y: usize,
}

pub struct Game {
    height: usize,
    width: usize,
    print_interval: Duration,
    board: Vec<Vec<u8>>,
    snake: VecDeque<Coordinate>,
    direction: (i32, i32),
    is_game_over: bool,
    score: i32,
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
            board: Vec::new(),
            snake: VecDeque::new(),
            direction: (1, 0),
            is_game_over: false,
            score: 0,
        };
    }

    pub fn set_target_fps(&mut self, fps: u64) {
        let frame_duration_ms = 1000 / fps;
        self.print_interval = Duration::from_millis(frame_duration_ms);
    }

    fn reset(&mut self) {
        self.board = vec![vec![EMPTY; self.width]; self.height];
        self.snake = VecDeque::new();
        self.direction = (1, 0);
        self.is_game_over = false;
        self.score = 0;

        let start_pos = Coordinate {
            x: 0,
            y: self.height / 2,
        };

        self.board[start_pos.y][start_pos.x] = SNAKE;
        self.snake.push_front(start_pos);
        self.spawn_new_food();
        Game::clear_screen();
    }

    pub async fn start(&mut self) {
        let mut last_print_time = Instant::now();
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
 
            let curr_time = Instant::now();

            if curr_time - last_print_time >= self.print_interval && !self.is_game_over {
                last_print_time = curr_time;

                self.update();
                self.draw();
            }

            sleep(Duration::from_millis(1)).await;
        }
    }

    fn update(&mut self) {
        let head = self.snake.front().unwrap();
        let next_head = Coordinate {
            x: (head.x as i32 + self.direction.0) as usize,
            y: (head.y as i32 + self.direction.1) as usize,
        };

        self.is_game_over = next_head.x >= self.width
            || next_head.y >= self.height
            || self.board[next_head.y][next_head.x] == SNAKE;

        if self.is_game_over {
            return;
        }

        let is_consuming_food = self.board[next_head.y][next_head.x] == FOOD;
        
        self.board[next_head.y][next_head.x] = SNAKE;
        self.snake.push_front(next_head);
        
        if is_consuming_food {
            self.score += 1;
            self.spawn_new_food();
        } else {
            let tail = self.snake.back().unwrap();

            self.board[tail.y][tail.x] = EMPTY;
            self.snake.pop_back();
        }
    }

    fn draw(&mut self) {
        if self.is_game_over {
            Game::clear_screen();
            println!("Score: {}\nGame Over!\nPress 'r' to restart or 'q' to quit.", self.score);
            return;
        }

        let mut s = format!("\x1B[HScore: {}\n", self.score);
        for row in &self.board {
            for cell in row {
                let val;

                if *cell == EMPTY {
                    val = "-  ";
                } else if *cell == FOOD {
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

    fn spawn_new_food(&mut self) {
        let mut rng = rand::thread_rng();

        loop {
            let x = rng.gen::<usize>() % self.width;
            let y = rng.gen::<usize>() % self.height;

            if self.board[y][x] != EMPTY {
                continue;
            }

            self.board[y][x] = FOOD;
            break;
        }
    }

    fn clear_screen() {
        // print!("\x1B[2J\x1B[1;1H");
        print!("\x1B[3J\x1B[2J\x1B[H");
        // print!("\x1B[2J\x1B[H");
        // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }
}
