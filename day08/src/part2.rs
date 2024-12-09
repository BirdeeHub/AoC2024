use std::fs::File;
use std::collections::HashMap;
use std::time::Instant;
use std::io::{self, BufRead, BufReader};
use std::env;
use itertools::Itertools;

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

#[derive(Debug, PartialEq, Clone, Copy)]
struct Position (usize,usize);
impl Position {
    pub fn slope_from(&self, other: &Position) -> (i32, i32) {
        (other.0 as i32 - self.0 as i32, other.1 as i32 - self.1 as i32)
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

    populate_antis(&mut mapgrid);

    let mut anticount = 0;

    for row in &mapgrid {
        for space in row {
            if ! space.antis.is_empty() {
                anticount += 1;
            }
        }
    }

    println!("Antinode count: {}", anticount);

    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}

fn get_pairs(mapgrid: &[Vec<MapSpace>]) -> HashMap<char, Vec<(Position, Position)>> {
    let mut antennas:HashMap<char, Vec<Position>> = HashMap::new();

    for (i, row) in mapgrid.iter().enumerate() {
        for (j, space) in row.iter().enumerate() {
            if let Some(freq) = space.tenna {
                antennas.entry(freq).or_default().push(Position(i,j));
            }
        }
    }

    let mut pairs:HashMap<char, Vec<(Position, Position)>> = HashMap::new();
    // Example continued from your code
    for (&freq, positions) in &antennas {
        if positions.len() < 2 {
            continue;
        }
        let combinations = positions
            .iter()
            .tuple_combinations() // Generate all unique pairs
            .map(|(a, b)| (*a, *b))
            .collect::<Vec<_>>();
        pairs.insert(freq, combinations);
    }
    pairs
}

fn populate_antis(mapgrid: &mut [Vec<MapSpace>]) {
    let pairmap = get_pairs(mapgrid);
    for (freq, pairs) in pairmap {
        for (a, b) in pairs {
            mapgrid[a.0][a.1].antis.push(freq);
            mapgrid[b.0][b.1].antis.push(freq);
            let (dx, dy) = a.slope_from(&b);
            let mut newx = (a.0 as i32) - dx;
            let mut newy = (a.1 as i32) - dy;
            while newx >= 0 && newx < mapgrid.len() as i32 && newy >= 0 && newy < mapgrid[newx as usize].len() as i32{
                mapgrid[newx as usize][newy as usize].antis.push(freq);
                newx -= dx;
                newy -= dy;
            }
            let (dx, dy) = b.slope_from(&a);
            let mut newx = (b.0 as i32) - dx;
            let mut newy = (b.1 as i32) - dy;
            while newx >= 0 && newx < mapgrid.len() as i32 && newy >= 0 && newy < mapgrid[newx as usize].len() as i32{
                mapgrid[newx as usize][newy as usize].antis.push(freq);
                newx -= dx;
                newy -= dy;
            }
        }
    }
}
