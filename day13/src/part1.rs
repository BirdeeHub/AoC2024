use std::fs::File;
use std::collections::HashMap;
use std::time::Instant;
use std::io::{self, BufRead, BufReader};
use std::env;
use regex::Regex;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32
}
impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point{x, y}
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

    let button_match = Regex::new(r"^Button A|B: X\+(\d+), Y\+(\d+)$").unwrap();
    let prize_match = Regex::new(r"^Prize: X=(\d+), Y=(\d+)$").unwrap();

    let mut results = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if ! line.is_empty() {
            if button_match.is_match(&line) {
                for (_, [id,dx,dy]) in button_match.captures_iter(&line).map(|c| c.extract()) {
                    results.push((dx.parse::<i32>().unwrap(), dy.parse::<i32>().unwrap()));
                }
            } else if prize_match.is_match(&line) {
                for (_, [x,y]) in prize_match.captures_iter(&line).map(|c| c.extract()) {
                    results.push((x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()));
                }
            }
        }
    }

    let machines:Vec<[Point;3]> = results.chunks(3).map(|chunk| {
        let mut res = Vec::new();
        for (x,y) in chunk {
            res.push(Point::new(*x, *y));
        }
        match (&res[0..2]).try_into() {
            Ok(arr) => arr,
            Err(_) => panic!("AOC input invalid, not all machines have 2 buttons and a prize")
        }
    }).collect();

    let mut total = 0;
    for [a, b, p] in machines {
        total += solve(a,b,p).unwrap_or(0);
    }

    println!("Part 1: {}", total);

    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}

// returns token count or none
fn solve(a: Point, b: Point, p: Point) -> Option<(usize)> {
    // TODO: Your algebra sucked and you didnt even read the whole problem...
    // A costs 3 and B costs 1
    // you didnt even solve for the right thing nor did you do it right...
    let canReach = (p.y * a.x - p.x * a.y) % (p.y * b.x - p.x * b.y) == 0;
    let Btimes = (p.y * a.x - p.x * a.y) / (p.y * b.x - p.x * b.y);
    let Atimes = (b.x - p.x * Btimes) / a.x;
    println!("{} {} {}", Atimes, Btimes, canReach);
}
