use std::fs::File;
use std::time::Instant;
use std::io::{self, BufRead, BufReader};
use std::env;
use std::ops::Add;
use regex::Regex;
use std::{thread, time::Duration};

#[derive(Debug, Copy, Clone)]
struct Vec2 {
    x: i32,
    y: i32
}
impl Vec2 {
    fn new(x: i32, y: i32) -> Vec2 {
        Vec2{x, y}
    }
    fn distance(&self, other: &Vec2) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
    }
}
impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Clone)]
struct Bot {
    p: Vec2,
    v: Vec2
}
impl Bot {
    fn move_bot(&mut self, w: i32, h: i32) {
        // get new_p, if its more, get that with mod, add max to shift it
        // then use mod again to wrap again if still bigger.
        let new_p = self.p + self.v;
        self.p = Vec2 {
            x: (new_p.x % w + w) % w,
            y: (new_p.y % h + h) % h,
        };
    }
}

pub fn run() -> io::Result<()> {
    let start = Instant::now();
    let args: Vec<String> = std::env::args().collect();
    let file = File::open(match args.get(1) {
        Some(fp) => fp.to_string(),
        _ => env::var("AOC_INPUT").expect("AOC_INPUT not set"),
    })?;
    let room_w = args.get(2).expect("room_w not set (arg 2)").parse().unwrap();
    let room_h = args.get(3).expect("room_h not set (arg 3)").parse().unwrap();
    let reader = BufReader::new(file);

    let bot_match = Regex::new(r"^p\=(\d+),(\d+) v\=(-\d+|\d+),(-\d+|\d+)$").unwrap();
    let mut bots = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if ! line.is_empty() && bot_match.is_match(&line) {
            for (_, [x,y,dx,dy]) in bot_match.captures_iter(&line).map(|c| c.extract()) {
                let p = Vec2::new(x.parse().unwrap(), y.parse().unwrap());
                let v = Vec2::new(dx.parse().unwrap(), dy.parse().unwrap());
                bots.push(Bot{p,v});
            }
        }
    }

    for i in 0..100000 {
        let mut room = vec![vec![false; room_w as usize]; room_h as usize];
        for bot in &mut bots {
            bot.move_bot(room_w, room_h);
            room[bot.p.y as usize][bot.p.x as usize] = true;
        }
        print_room(&room);
        let entropy = calculate_closeness(&bots.iter().map(|b| b.p).collect());
        if entropy < 45.0 {
            thread::sleep(Duration::from_millis(1000));
        }
        thread::sleep(Duration::from_millis(100));
        println!("{i},{entropy}");
        println!();
    }

    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}

fn print_room(room: &[Vec<bool>]) {
    for row in room {
        for cell in row {
            print!("{}", if *cell {"#"} else {"."});
        }
        println!();
    }
}

fn calculate_closeness(positions: &Vec<Vec2>) -> f64 {
    let mut distances = Vec::new();
    let n = positions.len();

    // pairwise distances
    for i in 0..n {
        for j in i + 1..n {
            distances.push(positions[i].distance(&positions[j]));
        }
    }
    if distances.is_empty() {
        return 0.0;
    }

    // average distance
    let total_distance: f64 = distances.iter().sum();
    total_distance / distances.len() as f64
}
