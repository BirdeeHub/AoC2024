use std::fs::File;
use std::time::Instant;
use std::io::{self, BufRead, BufReader};
use std::env;
use std::ops::Add;
use regex::Regex;

#[derive(Debug, Copy, Clone)]
struct Vec2 {
    x: i32,
    y: i32
}
impl Vec2 {
    fn new(x: i32, y: i32) -> Vec2 {
        Vec2{x, y}
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

    let room_w = 11;
    let room_h = 7;

    for bot in &mut bots {
        for _ in 0..100 {
            bot.move_bot(room_w as i32, room_h as i32);
        }
    }
    let mut room = vec![vec![0; room_w]; room_h];
    for bot in bots {
        room[bot.p.y as usize][bot.p.x as usize] += 1;
    }

    print_room(&room);

    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}

fn print_room(room: &[Vec<usize>]) {
    for row in room {
        for val in row {
            print!("{}", val);
        }
        println!();
    }
}
