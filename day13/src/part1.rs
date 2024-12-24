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

    let button_match = Regex::new(r"^Button (A|B): X\+(\d+), Y\+(\d+)$").unwrap();
    let prize_match = Regex::new(r"^Prize: X=(\d+), Y=(\d+)$").unwrap();

    let mut results = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if ! line.is_empty() {
            if button_match.is_match(&line) {
                for (_, [id,dx,dy]) in button_match.captures_iter(&line).map(|c| c.extract()) {
                    results.push((id.to_string(), dx.parse::<i32>().unwrap(), dy.parse::<i32>().unwrap()));
                }
            } else if prize_match.is_match(&line) {
                for (_, [x,y]) in prize_match.captures_iter(&line).map(|c| c.extract()) {
                    results.push(("P".to_string(), x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()));
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

    for m in machines {
        // TODO: Your algebra sucked and you didnt even read the whole problem...
        // A costs 3 and B costs 1
        // you didnt even solve for the right thing nor did you do it right...
        let canReach = (m["P"].y * m["A"].x - m["P"].x * m["A"].y) % (m["P"].y * m["B"].x - m["P"].x * m["B"].y) == 0;
        let Btimes = (m["P"].y * m["A"].x - m["P"].x * m["A"].y) / (m["P"].y * m["B"].x - m["P"].x * m["B"].y);
        let Atimes = (m["B"].x - m["P"].x * Btimes) / m["A"].x;
        println!("{} {} {}", Atimes, Btimes, canReach);
    }

    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}
