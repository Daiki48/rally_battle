use std::thread::{
    sleep,
    spawn,
};
use std::time::{
    Duration,
    SystemTime
};
use std::io::Stdin;
use std::sync::{
    Mutex,
    Arc,
};

struct Game {
    pub ball: f64,
    pub speed: f64,
}

impl Game {
    pub fn new() -> Self {
        Self {
            ball: 0f64,
            speed: 0.01f64,
        }
    }

    pub fn update(&mut self, is_swing: &Mutex<bool>) -> bool{
        self.ball += self.speed;
        let mut is_swing = is_swing.lock().unwrap();
        let is_hit_left = *is_swing && (-HALF_PADDLE_SIZE..HALF_PADDLE_SIZE).contains(&self.ball);
        let is_hit_right = *is_swing && (1.0 - HALF_PADDLE_SIZE..1.0 + HALF_PADDLE_SIZE).contains(&self.ball);
        if is_hit_left || is_hit_right {
            self.speed *= -1.0;
        }
        *is_swing = false;
        let out_left = self.ball < -HALF_PADDLE_SIZE;
        let out_right = 1.0 + HALF_PADDLE_SIZE < self.ball;
        !out_left && !out_right
    }

    pub fn strike_back(&mut self) {
        let is_left = self.ball < 0.5;
        let judgement = if is_left {
            self.ball.abs()
        } else {
            (self.ball - 1.0).abs()
        };
        let is_just_timing = judgement < HALF_PADDLE_SIZE / 2.0;
        self.speed *= -1.0 * if is_just_timing { 1.1 } else { 1.0 };
    }
}

const COAT_SIZE: i32 = 64;
const HALF_PADDLE_SIZE: f64 = 0.2 / 2.0;

fn draw(ball: f64) {
    let ball: i32 = (COAT_SIZE as f64 * ball).round() as i32;
    let mut buf: String = String::from(" ");
    buf += "║";
    for i in 0..COAT_SIZE {
        buf += if i == ball { "@" } else { " " };
    }
    buf += "║";
    println!("\x1B[1;1H{}", buf);
}

fn game_loop(game: &mut Game, is_swing: &Mutex<bool>) {
    let mut time: SystemTime = SystemTime::now();
    loop {
        if !game.update(is_swing){
            break;
        }
        draw(game.ball);
        time += Duration::from_nanos(16_666_667);
        if let Ok(dur) = time.duration_since(SystemTime::now()) {
            sleep(dur);
        }
    }
    println!("Game Over");
}

fn sub_main(is_swing: &Mutex<bool>) -> ! {
    let input: Stdin = std::io::stdin();
    let mut buf: String = String::new();
    loop {
        input.read_line(&mut buf).unwrap();
        *is_swing.lock().unwrap() = true;
    }
}

fn main() {
    // xterm control sequence
    println!("\x1B[2J"); // screen clear
    let is_swing: Arc<Mutex<bool>> = Default::default();
    {
        let is_swing = is_swing.clone();
        spawn(move || sub_main(&is_swing));
    }
    let mut game: Game = Game::new();
    game_loop(&mut game, &is_swing);
}
