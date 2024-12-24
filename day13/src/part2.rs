use std::fs::File;
use std::time::Instant;
use std::io::{self, BufRead, BufReader};
use std::env;
use regex::Regex;

#[derive(Debug, Copy, Clone)]
struct Vec2 {
    x: f64,
    y: f64
}
impl Vec2 {
    fn new(x: f64, y: f64) -> Vec2 {
        Vec2{x, y}
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

    let button_match = Regex::new(r"^Button [AB]: X\+(\d+), Y\+(\d+)$").unwrap();
    let prize_match = Regex::new(r"^Prize: X=(\d+), Y=(\d+)$").unwrap();

    let mut results = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if ! line.is_empty() {
            if button_match.is_match(&line) {
                for (_, [dx,dy]) in button_match.captures_iter(&line).map(|c| c.extract()) {
                    results.push((dx.parse::<f64>().unwrap(), dy.parse::<f64>().unwrap()));
                }
            } else if prize_match.is_match(&line) {
                for (_, [x,y]) in prize_match.captures_iter(&line).map(|c| c.extract()) {
                    results.push((x.parse::<f64>().unwrap(), y.parse::<f64>().unwrap()));
                }
            }
        }
    }

    let machines:Vec<[Vec2;3]> = results.chunks(3).map(|chunk| {
        let mut res = Vec::new();
        for (x,y) in chunk {
            res.push(Vec2::new(*x, *y));
        }
        match (&res[0..=2]).try_into() {
            Ok(arr) => arr,
            Err(_) => panic!("AOC input invalid, not all machines have 2 buttons and a prize")
        }
    }).collect();

    let mut total = 0;
    for [a, b, p] in &machines {
        let newp = Vec2::new(p.x+10000000000000., p.y+10000000000000.);
        total += solve(*a,*b,newp).unwrap_or(0);
    }

    println!("Part 1: {:?}", total);

    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}

// returns token count or none
fn solve(a: Vec2, b: Vec2, p: Vec2) -> Option<usize> {
    let at = (p.x * b.y - p.y * b.x) / (a.x * b.y - a.y * b.x);
    let bt = (p.x - a.x * at) / b.x;
    let tokens = 3. * at + bt;
    if bt.fract() != 0. || at.fract() != 0. {
        None
    } else {
        Some(tokens as usize)
    }
}
//find min value of pt to return
// 3 * at + bt = pt
// a.x * at + b.x * bt = p.x
// a.y * at + b.y * bt = p.y
