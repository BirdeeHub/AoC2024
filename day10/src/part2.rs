use std::fs::File;
use std::ops::{Deref, DerefMut};
use std::time::Instant;
use std::io::{self, BufRead, BufReader};
use std::env;

pub fn run() -> io::Result<()> {
    let start = Instant::now();
    let file = File::open(match std::env::args().collect::<Vec<String>>().get(1) {
        Some(fp) => fp.to_string(),
        _ => env::var("AOC_INPUT").expect("AOC_INPUT not set")
    })?;
    let reader = BufReader::new(file);

    let mut map = Map::new();
    for line in reader.lines() {
        let line = line?;
        let row:Vec<usize> = line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
        map.push(row);
    }
    let mut trailheads = Vec::new();
    for (i,row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == 0 {
                trailheads.push((Position::new(i,j),0));
            }
        }
    }

    for (th, count) in &mut trailheads {
        *count = calc_trails(&map, th, 0).len();
    }

    let mut finalcount = 0;
    for (_, count) in &trailheads {
        finalcount += count;
    }

    println!("Final count: {:?}", finalcount);

    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}

fn deduplicate_vec<T: PartialEq>(vec: Vec<T>) -> Vec<T> {
    let mut result = Vec::new();
    for item in vec {
        if !result.contains(&item) {
            result.push(item);
        }
    }
    result
}

#[derive(Debug,Clone,PartialEq)]
struct Position {
    row: usize,
    col: usize
}

impl Position {
    fn new(row: usize, col: usize) -> Position {
        Position { col, row }
    }
}

#[derive(Debug,Clone)]
struct Map(Vec<Vec<usize>>);

impl Map {
    const TO_CHECK: [(i32,i32); 4] = [(0,1),(0,-1),(1,0),(-1,0)];
    fn new() -> Map {
        Map(Vec::new())
    }
    fn neighbors_with_val(&self, pos: &Position, val: usize) -> Vec<Position> {
        let mut res = Vec::new();
        for (row,col) in Self::TO_CHECK {
            let newrow = (pos.row as i32)+row;
            let newcol = (pos.col as i32)+col;
            if newrow >= 0 && newcol >= 0 && newrow < self.len() as i32 && newcol < self[newrow as usize].len() as i32 {
                let newpos = Position::new(newrow as usize,newcol as usize);
                if self[newpos.row][newpos.col] == val {
                    res.push(newpos);
                }
            }
        }
        res
    }
}

impl Deref for Map {
    type Target = Vec<Vec<usize>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn calc_trails(map: &Map, th: &Position, level: usize) -> Vec<Position> {
    if level > 8 { return vec![th.clone()]; };
    let mut retval = Vec::new();
    let nextset = map.neighbors_with_val(th,level+1);
    for val in nextset {
        for item in calc_trails(map,&val,level+1) {
            retval.push(item);
        }
    }
    retval
}
