use std::fs::File;
use std::time::Instant;
use std::io::{self, BufRead, BufReader};
use std::env;
use regex::Regex;

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

    let machines:Vec<Vec<(String, usize, usize)>> = results.chunks(3).map(|chunk| chunk.to_vec()).collect();

    for machine in machines {
        println!("{:?}", machine);
    }

    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}
