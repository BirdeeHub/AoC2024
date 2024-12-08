use std::fs::File;
use std::time::Instant;
use std::io::{self, BufRead, BufReader};
use std::env;

#[derive(Debug, PartialEq)]
struct MapSpace {
    pub tenna: Option<char>,
    pub antis: Vec<char>,
}

impl MapSpace {
    pub fn new() -> MapSpace {
        MapSpace { tenna: None, antis: Vec::new() }
    }
}

pub fn run() -> io::Result<()> {
    let start = Instant::now();
    let args: Vec<String> = std::env::args().collect();
    let filepath = match args.get(1) {
        Some(fp) => fp.to_string(),
        _ => env::var("AOC_INPUT").expect("AOC_INPUT not set")
    };
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut mapgrid:Vec<Vec<MapSpace>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut row:Vec<MapSpace> = Vec::new();
        for c in line.chars() {
            let mut space = MapSpace::new();
            row.push(match c {
                '0'..='9' | 'a'..='z' | 'A'..='Z' => { space.tenna = Some(c); space },
                _ => space,
            });
        }
        mapgrid.push(row);
    }

    for row in mapgrid.iter() {
        println!("{:?}", row);
    }

    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}
