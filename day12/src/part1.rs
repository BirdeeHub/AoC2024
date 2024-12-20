use std::fs::File;
use std::time::Instant;
use std::io::{self, BufRead, BufReader};
use std::env;

pub fn run() -> io::Result<()> {
    let start = Instant::now();
    let args: Vec<String> = std::env::args().collect();
    let file = File::open(match args.get(1) {
        Some(fp) => fp.to_string(),
        _ => env::var("AOC_INPUT").expect("AOC_INPUT not set"),
    })?;
    let reader = BufReader::new(file);

    let mut garden:Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        garden.push(line?.chars().collect());
    }

    for row in garden {
        println!("{:?}", row);
    }

    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}
