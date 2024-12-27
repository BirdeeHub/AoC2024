use std::fs::File;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Deref, DerefMut};
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
            '<' => moves.push(Moves::L),
            '>' => moves.push(Moves::R),
            '^' => moves.push(Moves::U),
            'v' => moves.push(Moves::D),
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

#[derive(Debug,Clone,PartialEq)]
struct Room {
    map: Vec<Vec<Space>>,
    bot_pos: Vec2,
}
impl Room {
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
    fn get_pos(&self, p: Vec2) -> Option<Space> {
        if let Some(row) = self.get(p.y as usize) {
            row.get(p.x as usize).copied()
        } else {
            None
        }
    }
    fn set_pos(&mut self, p: Vec2, val: Space) {
        if let Some(row) = self.get_mut(p.y as usize) {
            if let Some(v) = row.get_mut(p.x as usize) {
                *v = val;
            }
        };
    }
    fn apply_move(&mut self, m: Moves) {
        let v = m.to_v();
        let mut newpos = self.bot_pos;
        while let Some(space) = self.get_pos(newpos) {
            match space {
                Space::Wall => break,
                Space::Empty => {
                    self.set_pos(self.bot_pos, Space::Empty);
                    self.set_pos(newpos, Space::Box);
                    self.bot_pos = self.bot_pos + v;
                    self.set_pos(self.bot_pos, Space::Robot);
                    break
                },
                Space::Box => {},
                Space::Robot => {},
            }
            newpos = newpos + v;
        }
    }
}
impl std::str::FromStr for Room {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Vec::new();
        let mut bot_pos: Option<Vec2> = None;
        for (j, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (i, c) in line.chars().enumerate() {
                match c {
                    '.' => row.push(Space::Empty),
                    '#' => row.push(Space::Wall),
                    'O' => row.push(Space::Box),
                    '@' => {
                        if bot_pos.is_some() { return Err("Multiple robots".to_string()); }
                        bot_pos = Some(Vec2::new(i as i32,j as i32));
                        row.push(Space::Robot);
                    },
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
        match bot_pos { 
            None => Err("Robot not found".to_string()),
            Some(bp) => Ok(Room { map, bot_pos: bp}),
        }
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

#[derive(Debug, Copy, Clone,PartialEq)]
struct Vec2 {
    x: i32,
    y: i32
}
impl Vec2 {
    fn new(x: i32, y: i32) -> Vec2 {
        Vec2{x, y}
    }
}
impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
#[derive(Debug,Clone,Copy)]
enum Moves {
    L,
    R,
    U,
    D,
}
impl Moves {
    fn to_v(self) -> Vec2 {
        match self {
            Moves::L => Vec2::new(-1,0),
            Moves::R => Vec2::new(1,0),
            Moves::U => Vec2::new(0,-1),
            Moves::D => Vec2::new(0,1),
        }
    }
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
