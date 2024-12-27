use std::fs::File;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub, Deref, DerefMut};
use std::time::Instant;
use std::io::{self, Read};
use std::env;

fn read_file(file_path: &str) -> io::Result<String> {
    let mut contents = String::new();
    File::open(file_path)?.read_to_string(&mut contents)?;
    Ok(contents)
}

fn make_doubled(instr: &str) -> String {
    let mut res = String::new();
    for c in instr.chars() {
        match c {
            '#' => res.push_str("##"),
            'O' => res.push_str("[]"),
            '.' => res.push_str(".."),
            '@' => res.push_str("@."),
            '\n' => res.push(c),
            _ => {},
        }
    }
    res
}

pub fn run() -> io::Result<()> {
    let start = Instant::now();
    let args: Vec<String> = std::env::args().collect();
    let filepath = match args.get(1) {
        Some(fp) => fp.to_string(),
        _ => env::var("AOC_INPUT").expect("AOC_INPUT not set")
    };
    let filetext = read_file(&filepath)?;
    let contents:Vec<&str> = filetext.split("\n\n").collect();
    let mut map:Room = make_doubled(contents[0]).parse().unwrap();
    let movestr = contents[1];
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
        println!();
        map.apply_move(m);
        println!("{}",map);
    }
    println!();
    println!("{}",map);
    println!("Part 2: {}", map.part2_total());
    println!("Time taken: {:?}", start.elapsed());
    Ok(())
}

#[derive(Debug,Clone,PartialEq)]
struct Room {
    map: Vec<Vec<Space>>,
    bot_pos: Vec2,
}
impl Room {
    fn part2_total(&self) -> usize {
        let mut total = 0;
        for (i, row) in self.iter().enumerate() {
            let mut lastbox: Option<Vec<Vec2>> = None;
            for (j, space) in row.iter().enumerate() {
                if let Space::Box(positions) = space.clone() {
                    let lbpos = positions.clone();
                    if let Some(ref lb) = lastbox {
                        if *lb != lbpos {
                            total += i*100+j;
                        }
                    } else {
                        total += i*100+j;
                    }
                    lastbox = Some(lbpos);
                }
            }
        }
        total
    }
    fn get_pos(&self, p: Vec2) -> Option<Space> {
        if let Some(row) = self.get(p.y as usize) {
            row.get(p.x as usize).cloned()
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
    fn check_move(&self, m: Moves, p: Vec2) -> Option<HashSet<Vec2>> {
        match self.get_pos(p) {
            Some(Space::Box(b)) => {
                match m {
                    Moves::U | Moves::D => {
                        let mut nexts = Some(HashSet::from_iter([p]));
                        for bp in b {
                            if let Some(ref mut n) = nexts {
                                let nr = self.check_move(m,bp+m.to_vec2());
                                match nr {
                                    Some(r) => {
                                        for v in r {
                                            n.insert(v);
                                        }
                                    },
                                    _ => nexts = None,
                                }
                            }
                        }
                        println!("{:?}",nexts);
                        nexts
                    },
                    _ => self.check_move(m,p + m.to_vec2()).map(|v|{
                        let mut n = v.clone();
                        n.insert(p);
                        n
                    }),
                }
            },
            Some(Space::Empty) => {
                Some(HashSet::new())
            },
            _ => None,
        }
    }
    fn apply_move(&mut self, m: Moves) {
        if let Some(boxes) = self.check_move(m,self.bot_pos+m.to_vec2()) {
            let mut new_boxes = Vec::new();
            for bp in boxes {
                println!("{:?}",bp);
                if let Space::Box(b) = self.get_pos(bp).expect("you returned a box that doesnt exist") {
                    self.set_pos(bp,Space::Empty);
                    new_boxes.push((bp+m.to_vec2(),Space::Box(b.iter().map(|p|*p+m.to_vec2()).collect())));
                }
            }
            for (pos,space) in new_boxes {
                self.set_pos(pos,space);
            }
            self.set_pos(self.bot_pos,Space::Empty);
            self.bot_pos = self.bot_pos + m.to_vec2();
            self.set_pos(self.bot_pos,Space::Robot);
        };
    }
}
impl std::str::FromStr for Room {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Vec::new();
        let mut bot_pos: Option<Vec2> = None;
        for (j, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            let mut in_box: Option<Vec<Vec2>> = None;
            for (i, c) in line.chars().enumerate() {
                match c {
                    '.' => row.push(Space::Empty),
                    '#' => row.push(Space::Wall),
                    'O' => {
                        if in_box.is_none() {
                            row.push(Space::Box(vec![Vec2::new(i as i32,j as i32)]))
                        } else {
                            return Err("Box in a box doesnt need to be on the map...".to_string());
                        }
                    },
                    '[' => {
                        in_box = Some(vec![Vec2::new(i as i32,j as i32)]);
                    },
                    '=' => {
                        if let Some(ref mut b) = in_box {
                            b.push(Vec2::new(i as i32,j as i32));
                        } else {
                            return Err("No start to box".to_string());
                        }
                    },
                    ']' => {
                        if let Some(ref mut b) = in_box {
                            b.push(Vec2::new(i as i32,j as i32));
                            let blen = b.len();
                            for _ in 0..blen {
                                row.push(Space::Box(b.clone()));
                            }
                            in_box = None;
                        } else {
                            return Err("Unmatched end of box".to_string());
                        }
                    },
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
        for (i, row) in map.iter().enumerate() {
            if row.len() != x {
                println!("{} != {} at {}",row.len(),x,i);
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
        let mut j = 0;
        while let Some(row) = iter.next() {
            for (i, space) in row.iter().enumerate() {
                res.push_str(match space {
                    Space::Robot => "@",
                    Space::Box(locs) => {
                        if let Some(l) = locs.first() {
                            if l.x == i as i32 && l.y == j {
                                if locs.len() > 1 {
                                    "["
                                } else {
                                    "O"
                                }
                            } else if let Some(l) = locs.last() {
                                if l.x == i as i32 && l.y == j {
                                    "]"
                                } else {
                                    "="
                                }
                            } else {
                                "O"
                            }
                        } else {
                            "O"
                        }
                    },
                    Space::Wall => "#",
                    Space::Empty => ".",
                });
            }
            if iter.len() > 0 {
                res.push('\n');
            }
            j += 1;
        }
        fmt.write_str(&res)
    }
}
#[derive(Debug, Copy, Clone,PartialEq,Eq,Hash)]
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
impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
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
    fn to_vec2(self) -> Vec2 {
        match self {
            Moves::L => Vec2::new(-1,0),
            Moves::R => Vec2::new(1,0),
            Moves::U => Vec2::new(0,-1),
            Moves::D => Vec2::new(0,1),
        }
    }
}
#[derive(Debug,Clone,PartialEq)]
enum Space {
    Empty,
    Wall,
    Box(Vec<Vec2>),
    Robot,
}
