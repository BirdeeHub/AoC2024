use std::fs::File;
use std::time::Instant;
use std::io::{self, BufRead, BufReader};
use std::env;

struct Position {
    row: usize,
    col: usize
}

impl Position {
    fn new(row: usize, col: usize) -> Position {
        Position { col, row }
    }
}

pub fn run() -> io::Result<()> {
    let start = Instant::now();
    let filepath = match std::env::args().collect::<Vec<String>>().get(1) {
        Some(fp) => fp.to_string(),
        _ => env::var("AOC_INPUT").expect("AOC_INPUT not set")
    };
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut map = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let row:Vec<usize> = line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
        map.push(row);
    }
    let mut trailheads = Vec::new();
    for (i,row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == 0 {
                trailheads.push(Position::new(i,j));
            }
        }
    }

    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}
