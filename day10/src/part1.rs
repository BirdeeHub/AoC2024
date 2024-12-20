use std::fs::File;
use std::time::Instant;
use std::io::{self, BufRead, BufReader};
use std::env;

pub fn run() -> io::Result<()> {
    let start = Instant::now();
    let filepath = match std::env::args().collect::<Vec<String>>().get(1) {
        Some(fp) => fp.to_string(),
        _ => env::var("AOC_INPUT").expect("AOC_INPUT not set")
    };
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut map:Vec<Vec<usize>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let row:Vec<usize> = line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
        map.push(row);
    }

    for row in map.iter() {
        println!("{:?}", row);
    }

    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}
