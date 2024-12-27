use std::fs::File;
use std::time::Instant;
use std::io::{self, BufRead, BufReader};
use std::env;

#[derive(Debug,Copy,Clone,PartialEq)]
enum Space {
    Empty,
    Wall,
}

pub fn run() -> io::Result<()> {
    let start = Instant::now();
    let file = File::open(match std::env::args().collect::<Vec<String>>().get(1) {
        Some(fp) => fp.to_string(),
        _ => env::var("AOC_INPUT").expect("AOC_INPUT not set")
    })?;
    let reader = BufReader::new(file);

    let mut map = Vec::new();
    let mut start_pos = (0,0);
    let mut end_pos = (0,0);
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        let mut row = Vec::new();
        for (j, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    row.push(Space::Wall);
                },
                '.' => {
                    row.push(Space::Empty);
                },
                'S' => {
                    start_pos = (i,j);
                    row.push(Space::Empty);
                },
                'E' => {
                    end_pos = (i,j);
                    row.push(Space::Empty);
                },
                _ => {},
            }
        }
        map.push(row);
    }
    println!("{:?} {:?}", start_pos, end_pos);
    for row in &map {
        println!("{:?}", row);
    }

    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}
