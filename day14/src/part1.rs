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
    fn get_quad(&self, w: i32, h: i32) -> Option<Quads> {
        if self.p.x < w / 2 && self.p.y < h / 2 {
            Some(Quads::NE)
        } else if self.p.x < w / 2 && self.p.y > h / 2 {
            Some(Quads::SE)
        } else if self.p.x > w / 2 && self.p.y < h / 2 {
            Some(Quads::NW)
        } else if self.p.x > w / 2 && self.p.y > h / 2 {
            Some(Quads::SW)
        } else {
            // on the boundary
            None
        }
    }
}

enum Quads {
    NW,
    NE,
    SW,
    SE,
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

    let mut ne = 0;
    let mut se = 0;
    let mut sw = 0;
    let mut nw = 0;

    for bot in &mut bots {
        for _ in 0..100 {
            bot.move_bot(room_w, room_h);
        }
        let Some(q) = bot.get_quad(room_w, room_h) else { continue; };
        match q {
            Quads::NW => nw += 1,
            Quads::NE => ne += 1,
            Quads::SW => sw += 1,
            Quads::SE => se += 1
        }
    }

    println!("ne: {}, nw: {}, se: {}, sw: {}", ne, nw, se, sw);

    println!("Part 1: {}", ne * nw * se * sw);

    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}
