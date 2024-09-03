use clap::{Command, Arg};
use crossterm::{
    cursor,
    event::{self, KeyCode, KeyEvent, KeyModifiers},
    terminal::{self, ClearType},
    ExecutableCommand, QueueableCommand
};
use std::collections::VecDeque;
use std::io::{self, Write};
use std::time::{Duration, Instant};
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use signal_hook::iterator::Signals;
use signal_hook::consts::signal::{SIGINT, SIGTERM};
use std::process::exit;

struct GameState {
    player_y: i32,
    obstacles: VecDeque<u32>,
    score: u32,
    highscore: u32,
    obstacle_speed: u32,
    falling: bool,
    player_velocity: i32,
    cheats: bool,
}

impl GameState {
    fn new(cheats: bool) -> GameState {
        GameState {
            player_y: 0,
            obstacles: VecDeque::new(),
            score: 0,
            highscore: 0,
            obstacle_speed: 1,
            falling: false,
            player_velocity: 0,
            cheats,
        }
    }

    fn update(&mut self, frame_width: u32, frame_height: u32) {
        // Update obstacles
        if self.obstacles.front().map_or(false, |&x| x == 0) {
            self.obstacles.pop_front();
            self.obstacles.push_back(frame_width - 2);
            self.score += 1;

            if self.score > self.highscore {
                self.highscore = self.score;
            }
        }

        for obstacle in &mut self.obstacles {
            *obstacle = obstacle.saturating_sub(self.obstacle_speed);
        }

        // Update player position
        if self.falling {
            self.player_velocity += 1; // Simulate gravity
            let new_y = self.player_y + self.player_velocity;
            if new_y >= frame_height as i32 - 3 {
                self.player_y = frame_height as i32 - 3;
                self.falling = false;
                self.player_velocity = 0; // Reset velocity when hitting the ground
            } else if new_y < 0 {
                self.player_y = 0;
                self.falling = false;
                self.player_velocity = 0; // Reset velocity when hitting the top
            } else {
                self.player_y = new_y;
            }
        }

        if !self.cheats && self.check_collision(frame_width, frame_height) {
            println!("Game Over! Score: {}", self.score);
            thread::sleep(Duration::from_secs(2));
            self.reset();
        }
    }

    fn check_collision(&self, _frame_width: u32, _frame_height: u32) -> bool {
        if self.cheats {
            return false;
        }

        let player_x = 2;
        for &obstacle in &self.obstacles {
            if obstacle <= player_x + 1 && obstacle + 1 >= player_x {
                return true;
            }
        }
        false
    }

    fn reset(&mut self) {
        self.score = 0;
        self.obstacles.clear();
        self.player_y = 0;
        self.falling = false;
        self.player_velocity = 0;
    }
}


fn draw_frame(stdout: &mut io::Stdout, frame_width: u32, frame_height: u32) {
    let top_left = "╭";
    let top_right = "╮";
    let bottom_left = "╰";
    let bottom_right = "╯";
    let horizontal = "─";
    let vertical = "│";

    stdout.queue(cursor::MoveTo(0, 0)).unwrap();
    print!("{}", top_left);
    for _ in 0..frame_width - 2 {
        print!("{}", horizontal);
    }
    print!("{}", top_right);
    println!();

    for _ in 1..frame_height - 2 {
        print!("{}", vertical);
        stdout.queue(cursor::MoveTo(frame_width as u16 - 1, 0)).unwrap();
        print!("{}", vertical);
        println!();
    }

    print!("{}", bottom_left);
    for _ in 0..frame_width - 2 {
        print!("{}", horizontal);
    }
    print!("{}", bottom_right);
    println!();
}

fn draw_score(stdout: &mut io::Stdout, score: u32, highscore: u32) {
    stdout.queue(cursor::MoveTo(2, 1)).unwrap();
    write!(stdout, "Score: {}  Highscore: {}", score, highscore).unwrap();
}

fn draw_ground_line(stdout: &mut io::Stdout, frame_width: u32) {
    stdout.queue(cursor::MoveTo(0, (frame_width as i32 - 2) as u16)).unwrap();
    for _ in 0..frame_width {
        print!("─");
    }
    println!();
}

fn clear_screen(stdout: &mut io::Stdout) {
    stdout.queue(terminal::Clear(ClearType::All)).unwrap();
}

fn main() {
    let command = Command::new("Dino CLI Game")
        .version("1.0")
        .author("Ihr Name")
        .about("Ein einfaches Dino-Spiel")
        .arg(
            Arg::new("fps")
                .long("fps")
                .value_parser(clap::value_parser!(u32))
                .default_value("120")
                .help("Setzt die Frames pro Sekunde"),
        )
        .arg(
            Arg::new("max_obstacles")
                .long("max_obstacles")
                .value_parser(clap::value_parser!(u32))
                .default_value("1")
                .help("Setzt die maximale Anzahl der Hindernisse"),
        )
        .arg(
            Arg::new("cheats")
                .long("cheats")
                .action(clap::ArgAction::SetTrue)
                .help("Aktiviert Cheats"),
        )
        .get_matches();

    let fps: u32 = command.get_one::<u32>("fps").copied().unwrap_or(120);
    let max_obstacles: u32 = command.get_one::<u32>("max_obstacles").copied().unwrap_or(1);

    let mut stdout = io::stdout();
    let mut game_state = GameState::new(command.get_flag("cheats"));

    let (frame_width, frame_height) = terminal::size().unwrap();
    let frame_width = frame_width.into();
    let frame_height = frame_height.into();

    for _ in 0..max_obstacles {
        game_state.obstacles.push_back(frame_width - 2);
    }

    terminal::enable_raw_mode().unwrap();
    stdout.execute(terminal::Clear(ClearType::All)).unwrap();

    let target_fps = fps;
    let delay = Duration::from_secs_f32(1.0 / target_fps as f32);

    let stop_signal = Arc::new(AtomicBool::new(false));
    let stop_signal_clone = stop_signal.clone();

    thread::spawn(move || {
        let mut signals = Signals::new(&[SIGINT, SIGTERM]).unwrap();
        for _ in signals.forever() {
            stop_signal_clone.store(true, Ordering::SeqCst);
        }
    });

    loop {
        if stop_signal.load(Ordering::SeqCst) {
            clear_screen(&mut stdout);
            println!("Beendet! Highscore: {}", game_state.highscore);
            break;
        }

        let start_time = Instant::now();

        stdout.queue(terminal::Clear(ClearType::All)).unwrap();
        draw_frame(&mut stdout, frame_width, frame_height);
        draw_score(&mut stdout, game_state.score, game_state.highscore);
        draw_ground_line(&mut stdout, frame_width);

        let player_y_position = frame_height as i32 - 3 - game_state.player_y;
        stdout.queue(cursor::MoveTo(2, player_y_position as u16)).unwrap();
        write!(stdout, "🦖").unwrap();

        for obstacle in &game_state.obstacles {
            stdout.queue(cursor::MoveTo(*obstacle as u16, (frame_height - 3) as u16)).unwrap();
            write!(stdout, "🌵").unwrap();
        }

        stdout.flush().unwrap();

        if event::poll(Duration::from_millis(0)).unwrap() {
            if let event::Event::Key(KeyEvent { code, modifiers, .. }) = event::read().unwrap() {
                match code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        clear_screen(&mut stdout);
                        println!("Goodbye! Highscore: {}", game_state.highscore);
                        exit(0);
                    }
                    KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                        clear_screen(&mut stdout);
                        println!("Goodbye! Highscore: {}", game_state.highscore);
                        exit(0);
                    }
                    KeyCode::Char(' ') => {
                        if !game_state.falling {
                            game_state.falling = true;
                            game_state.player_velocity = -1; // Erhöhe die Start-Sprunggeschwindigkeit
                        }
                    }
                    _ => {}
                }
            }
        }

        game_state.update(frame_width, frame_height);

        let elapsed_time = Instant::now().duration_since(start_time);
        let sleep_duration = delay.saturating_sub(elapsed_time);
        thread::sleep(sleep_duration);
    }

    terminal::disable_raw_mode().unwrap();
    clear_screen(&mut stdout);
    println!("Goodbye! Highscore: {}", game_state.highscore);
}