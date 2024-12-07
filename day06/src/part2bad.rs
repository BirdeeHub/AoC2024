use std::fs::File;
use std::collections::HashSet;
use std::time::Instant;
use std::io::{self, BufRead, BufReader};
use std::env;

use crate::types::*;

fn deduplicate_vec<T: Eq + std::hash::Hash>(vec: Vec<T>) -> Vec<T> {
    let set: HashSet<_> = vec.into_iter().collect();
    set.into_iter().collect()
}

pub fn run() -> io::Result<usize> {
    let start = Instant::now();
    let inputvar = env::var("AOC_INPUT").expect("AOC_INPUT not set");
    let file = File::open(inputvar)?;
    let reader = BufReader::new(file);

    let mut room:Vec<Vec<RoomSpace>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut row:Vec<RoomSpace> = Vec::new();
        for c in line.chars() {
            row.push(match c {
                '^' => RoomSpace::Guard(Direction::Up),
                '#' => RoomSpace::Obstacle,
                _ => RoomSpace::Empty,
            });
        }
        room.push(row);
    }
    // fix x and y...
    let mut newroom = Vec::new();
    for i in 0..room[0].len() {
        let mut newrow = Vec::new();
        room.iter().for_each(|row|newrow.push(row[i].clone()));
        newroom.push(newrow);
    };
    room = newroom;

    let mut obstacles = Vec::new();
    for i in 0..room[0].len() {
        for j in 0..room[0].len() {
            if let Some(obs) = check_for_loop(&mut room.clone(), i, j) {
                obstacles.push(obs);
            }
        }
    }
    obstacles = deduplicate_vec(obstacles);

    println!("locations: {:?}",obstacles);
    println!("number: {:?}",obstacles.len());
    
    println!("Time taken: {:?}", start.elapsed());

    Ok(obstacles.len())
}

fn get_newspace(room: &[Vec<RoomSpace>], pos: (usize,usize), direction: &Direction) -> Option<(usize, usize)> {
    match direction {
        Direction::Up => {
            if pos.1 > 0 { Some((pos.0, pos.1 - 1)) } else { None }
        },
        Direction::Down => {
            if pos.1 < room[pos.0].len() -1 { Some((pos.0, pos.1 + 1)) } else { None }
        },
        Direction::Right => {
            if pos.0 < room.len() - 1 { Some((pos.0 + 1, pos.1)) } else { None }
        },
        Direction::Left => {
            if pos.0 > 0 { Some((pos.0 - 1, pos.1)) } else { None }
        },
    }
}

fn get_newspace_with_obstacle(room: &[Vec<RoomSpace>], pos: (usize,usize), direction: &Direction) -> Option<(Direction,(usize, usize))> {
    if let Some(newplace) = get_newspace(room, pos, direction) {
        if room[newplace.0][newplace.1] == RoomSpace::Obstacle {
            get_newspace_with_obstacle(room, pos, &turn_right(direction))
        } else {
            Some((direction.clone(),newplace))
        }
    } else {
        None
    }
}

fn turn_right(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn check_for_loop(room: &mut [Vec<RoomSpace>], obsx: usize, obsy: usize) -> Option<(usize,usize)> {
    if room[obsx][obsy] == RoomSpace::Obstacle {
        return None;
    }
    room[obsx][obsy] = RoomSpace::Obstacle;
    let mut continue_moving = true;
    let mut checkpoints = Vec::new();
    let mut checktrail = Vec::new();
    while continue_moving {
        continue_moving = move_guard(room, &mut checktrail);
        if continue_moving && checkpoints.contains(checktrail.last().unwrap()) {
            println!("LOOP! {:?} obs: {} {}", checktrail.last().unwrap(),obsx,obsy);
            return Some((obsx,obsy))
        }
        if continue_moving {
            checkpoints.push(checktrail.last().unwrap().clone());
        }
    }
    None
}

fn move_guard(room: &mut [Vec<RoomSpace>], trail: &mut Vec<(Direction,(usize,usize))>) -> bool {
    let mut guard_pos = (room.len(),room[0].len());
    let mut direction = Direction::Up;
    for (i, _) in room.iter().enumerate() {
        for (j, item) in room[i].iter().enumerate() {
            match item {
                RoomSpace::Guard(dir) => {
                    guard_pos = (i,j);
                    direction = dir.clone();
                },
                _ => continue,
            }
        }
    }
    if guard_pos.0 >= room.len() || guard_pos.1 >= room[0].len() {
        return false
    }
    room[guard_pos.0][guard_pos.1] = RoomSpace::Visited;
    trail.push((direction.clone(),guard_pos));
    if let Some((dir,newspace)) = get_newspace_with_obstacle(room, guard_pos, &direction) {
        room[newspace.0][newspace.1] = RoomSpace::Guard(dir.clone());
        true
    } else {
        false
    }
}
