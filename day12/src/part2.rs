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

    let mut grid:Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        grid.push(line?.chars().collect());
    }
    let mut res = Garden(Vec::new());
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            let pos = Position(i, j, *c);
            if !res.contains_position(&pos) {
                let mut new_region = Region(Vec::new());
                let mut visited = Vec::new();
                new_region.populate(&grid, &pos, &mut visited);
                res.push(new_region);
            }
        }
    }

    let ret = res.get_cost();

    println!("Part 2: {}", ret);

    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Position(usize, usize, char);
#[derive(Debug, PartialEq, Clone)]
struct Plot {
    pos: Position,
    edges: usize,
}
#[derive(Debug, PartialEq, Clone)]
struct Region(Vec<Plot>);
impl Region {
    const NEIGHBOURS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    fn populate(&mut self, field: &[Vec<char>], pos: &Position, visited: &mut Vec<Position>) {
        if visited.contains(pos) {
            return;
        }
        visited.push(*pos);
        let mut plot = Plot {
            pos: *pos,
            edges: 0,
        };
        for (dx, dy) in Self::NEIGHBOURS {
            let newrow = pos.0 as isize + dx;
            let newcol = pos.1 as isize + dy;
            if newrow < 0 || newcol < 0 || newrow >= field.len() as isize || newcol >= field[newrow as usize].len() as isize {
                plot.edges += 1;
            } else if let Some(neighbor_char) = field.get(newrow as usize).and_then(|row| row.get(newcol as usize)) {
                if *neighbor_char != pos.2 {
                    plot.edges += 1;
                } else {
                    self.populate(field, &Position(newrow as usize, newcol as usize, *neighbor_char), visited);
                }
            }
        }
        self.push(plot);
    }
    fn calc_cost(&self) -> u64 {
        // perimeter * area
        self.iter().fold(0, |acc, plot| acc + (plot.edges as u64)) * (self.len() as u64)
        // TODO: fix for part 2 so that its sides * area instead
        // where sides are contiguous edges that are on the same line
    }
}


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
#[derive(Debug, PartialEq, Clone)]
struct Garden(Vec<Region>);
impl Garden {
    fn contains_position(&self, pos: &Position) -> bool {
        self.iter().any(|region| {
            region.iter().any(|plot| plot.pos == *pos)
        })
    }
    fn get_cost(&self) -> u64 {
        self.iter().map(|v|v.calc_cost()).sum::<u64>()
    }
}
impl Deref for Garden {
    type Target = Vec<Region>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Garden {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
