use std::fs::File;
use std::collections::HashMap;
use std::time::Instant;
use std::io::{self, BufRead, BufReader};
use std::env;
use regex::Regex;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: usize,
    y: usize
}
impl Point {
    fn new(x: usize, y: usize) -> Point {
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

    let button_match = Regex::new(r"^Button (A|B): X\+(\d+), Y\+(\d+)$").unwrap();
    let prize_match = Regex::new(r"^Prize: X=(\d+), Y=(\d+)$").unwrap();

    let mut results = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if ! line.is_empty() {
            if button_match.is_match(&line) {
                for (_, [id,dx,dy]) in button_match.captures_iter(&line).map(|c| c.extract()) {
                    results.push((id.to_string(), dx.parse::<usize>().unwrap(), dy.parse::<usize>().unwrap()));
                }
            } else if prize_match.is_match(&line) {
                for (_, [x,y]) in prize_match.captures_iter(&line).map(|c| c.extract()) {
                    results.push(("P".to_string(), x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()));
                }
            }
        }
    }

    let machines:Vec<HashMap<String,Point>> = results.chunks(3).map(|chunk| {
        let mut res = HashMap::new();
        for (id,x,y) in chunk {
            res.insert(id.clone(), Point::new(*x, *y));
        }
        res
    }).collect();

    /*
        canReach = (m[P].y * m[A].x - m[P].x * m[A].y) % (m[P].y * m[B].x - m[P].x * m[B].y) != 0
        Btimes = (m[P].y * m[A].x - m[P].x * m[A].y) / (m[P].y * m[B].x - m[P].x * m[B].y)
        Atimes = (m[B].x - m[P].x * Btimes) / m[A].x
    */

    for m in machines {
        println!("{:?}", m);
    }

    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}
