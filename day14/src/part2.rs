use std::fs::File;
use std::time::Instant;
use std::io::{self, BufRead, BufReader};
use std::env;
use std::ops::Add;
use regex::Regex;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::{thread, time::Duration};

fn find_tree(bots: &[Bot],room: &[Vec<bool>]) -> bool {
    let positions = bots.iter().map(|b| b.p).collect::<Vec<Vec2>>();
    false
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

    let mut hashes = vec![calculate_hash(&bots)];
    let mut i = 0;
    loop {
        let mut room = vec![vec![false; room_w as usize]; room_h as usize];
        i += 1;
        for bot in &mut bots {
            bot.move_bot(room_w, room_h);
            room[bot.p.y as usize][bot.p.x as usize] = true;
        }
        print_room(&room);
        if find_tree(&bots, &room) {
            println!("found tree at: {i}");
            thread::sleep(Duration::from_millis(2000));
        }
        let hash = calculate_hash(&bots);
        if hashes.contains(&hash) {
            println!("cycled at: {i}");
            break;
        } else {
            hashes.push(hash);
            println!("{i}\n");
        };
    }

    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
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

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
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

fn print_room(room: &[Vec<bool>]) {
    for row in room {
        for cell in row {
            print!("{}", if *cell {"#"} else {"."});
        }
        println!();
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
