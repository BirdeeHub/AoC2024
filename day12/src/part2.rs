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
    fn calc_cost(&self, size: (usize,usize)) -> u64 {
        let area = self.len() as u64;
        // TODO: fix for part 2 so that its corners * area instead
        let mut acutes = Vec::new();
        let perimeter = self.iter().fold(0, |acc, plot| acc + (match plot.edges as u64 {
            4 => 4,
            3 => 2,
            2 => {
                //TODO: find acute and oblique corners if any, if acute beware of duplicates
                let outers = self.go_past_edges(&plot.pos);
                for (x, y) in outers { //<- will not iterate more times than there are edges, and they are the possible locations of an acute
                    if acutes.contains(&(x, y)) || x >= size.0 || y >= size.1 {
                        continue;
                    };
                    //TODO: check if acute angle
                    // a 1 edge plot is acute if it has a neighbor with 0 edges and a neighbor with 1+ edges
                    // perpendicular to the line between current and outsidespace
                    // place the location of the external space past the 1 edge into the acutes list, return 1
                }
                // TODO: check oblique corners 
            },
            1 => {
                let outers = self.go_past_edges(&plot.pos);
                for (x, y) in outers { //<- will not iterate more times than there are edges, and they are the possible locations of an acute
                    if acutes.contains(&(x, y)) || x >= size.0 || y >= size.1 {
                        continue;
                    };
                    //TODO: check if acute angle
                    // a 1 edge plot is acute if it has a neighbor with 0 edges and a neighbor with 1+ edges
                    // perpendicular to the line between current and outsidespace
                    // place the location of the external space past the 1 edge into the acutes list, return 1
                }
            },
            0 => 0,
        }));
        perimeter * area
    }

    fn go_past_edges(&self, pos: &Position) -> Vec<(usize, usize)> {
        let mut outers = Vec::new();
        for (dx, dy) in Self::NEIGHBOURS {
            let newrow = pos.0 as isize + dx;
            let newcol = pos.1 as isize + dy;
            if newrow < 0 || newcol < 0 {
                let mut found = false;
                for plot in self.iter() {
                    if plot.pos.0 == newrow as usize && plot.pos.1 == newcol as usize {
                        found = true;
                        break;
                    }
                }
                if ! found {
                    outers.push((newrow as usize, newcol as usize));
                }
            }
        }
        outers
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
        self.iter().map(|v|v.calc_cost((self.len(),self[0].len()))).sum::<u64>()
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
