use std::fs::File;
use std::ops::{Deref, DerefMut};
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

#[derive(Debug, PartialEq, Clone, Copy)]
struct Position(usize, usize);
#[derive(Debug, PartialEq, Clone)]
struct Plot {
    pos: Position,
    edges: usize,
}
#[derive(Debug, PartialEq, Clone)]
struct Region(Vec<Plot>);
impl Deref for Region {
    type Target = Vec<Plot>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Region {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
