use std::fs::File;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::time::Instant;
use std::io::{self, Read};
use std::env;

fn read_file(file_path: &str) -> io::Result<String> {
    let mut contents = String::new();
    File::open(file_path)?.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn run() -> io::Result<()> {
    let start = Instant::now();
    let args: Vec<String> = std::env::args().collect();
    let filepath = match args.get(1) {
        Some(fp) => fp.to_string(),
        _ => env::var("AOC_INPUT").expect("AOC_INPUT not set")
    };
    let contents = read_file(&filepath).unwrap();
    let inparts:Vec<&str> = contents.split("\n\n").collect();
    let mut map:Room = inparts[0].parse().unwrap();
    let movestr = inparts[1];
    let mut moves = Vec::new();
    for c in movestr.chars() {
        match c {
            '<' => moves.push(Move::L),
            '>' => moves.push(Move::R),
            '^' => moves.push(Move::U),
            'v' => moves.push(Move::D),
            _ => {},
        }
    }
    println!("{}",map);
    println!("Moves: {:?}", moves);
    for m in moves {
        map.apply_move(m);
    }
    println!();
    println!("{}",map);
    println!("Part 1: {}", map.part1_total());
    println!("Time taken: {:?}", start.elapsed());
    Ok(())
}

#[derive(Debug,Clone,Copy)]
enum Move {
    L,
    R,
    U,
    D,
}
#[derive(Debug,Clone,Copy,PartialEq)]
enum Space {
    Empty,
    Wall,
    Box,
    Robot,
}
impl Display for Space {
    fn fmt(&self, fmt:&mut Formatter) -> Result<(), std::fmt::Error> {
        fmt.write_str(match self {
            Space::Robot => "@",
            Space::Box => "O",
            Space::Wall => "#",
            Space::Empty => ".",
        })
    }
}

#[derive(Debug,Clone,PartialEq)]
struct Room {
    map: Vec<Vec<Space>>,
    bot_pos: Option<(usize,usize)>,
}
impl Room {
    pub fn new() -> Room {
        Room { map: Vec::new(), bot_pos: None }
    }
    fn part1_total(&self) -> usize {
        let mut total = 0;
        for (i, row) in self.iter().enumerate() {
            for (j, space) in row.iter().enumerate() {
                if let Space::Box = space {
                    total += i*100+j;
                }
            }
        }
        total
    }
    fn apply_move(&mut self, m: Move) {
        //TODO: this
        match m {
            Move::L => {
            },
            Move::R => {
            },
            Move::U => {
            },
            Move::D => {
            },
        };
    }
}
impl Deref for Room {
    type Target = Vec<Vec<Space>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}
impl DerefMut for Room {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}
impl Display for Room {
    fn fmt(&self, fmt:&mut Formatter) -> Result<(), std::fmt::Error> {
        let mut res = String::new();
        let mut iter = self.iter();
        while let Some(row) = iter.next() {
            for space in row.iter() {
                res.push_str(&format!("{}",space));
            }
            if iter.len() > 0 {
                res.push('\n');
            }
        }
        fmt.write_str(&res)
    }
}
impl std::str::FromStr for Room {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Room::new();
        for line in s.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                match c {
                    '.' => row.push(Space::Empty),
                    '#' => row.push(Space::Wall),
                    'O' => row.push(Space::Box),
                    '@' => row.push(Space::Robot),
                    _ => {},
                }
            }
            if row.is_empty() { continue; }
            map.push(row);
        }
        let mut x = map[0].len();
        for row in map.iter() {
            if row.len() != x {
                return Err("Map is irregular".to_string());
            } else {
                x = row.len();
            }
        }
        Ok(map)
    }
}
