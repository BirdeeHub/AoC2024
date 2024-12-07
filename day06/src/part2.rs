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

pub fn run(expected:Vec<(usize,usize)>) -> io::Result<()> {
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

    let mut room_with_guard = room.clone();
    let guardless_room: Vec<Vec<RoomSpace>> = room
        .iter()
        .map(|row| {
            row.iter()
                .map(|space| match space {
                    RoomSpace::Guard(_) => RoomSpace::Empty,
                    _ => space.clone(),
                })
                .collect()
        })
        .collect();

    let mut trail:Vec<(Direction,(usize,usize))> = Vec::new();
    let mut continue_moving = true;
    while continue_moving {
        continue_moving = move_guard(&mut room_with_guard, &mut trail);
        //print_room(&room_with_guard, 250)
    }

    let mut obstacles = Vec::new();
    for (i,(dir, (x,y))) in trail.iter().enumerate() {
        println!("{} / {}",i+1,trail.len());
        if let Some(obs) = check_right_for_loop(&mut guardless_room.clone(), (*x,*y), dir) {
            obstacles.push(obs);
        }
    }
    obstacles = deduplicate_vec(obstacles);

    println!("locations: {:?}",obstacles);
    println!("number: {:?}",obstacles.len());
    println!( "{}",
        if obstacles.len() != expected.len() {
            let extra: Vec<_> = obstacles.iter().filter(|v|!expected.contains(v)).collect();
            let missing: Vec<_> = expected.iter().filter(|v|!obstacles.contains(v)).collect();
            format!(
                "FAIL, expected {}{}{}",
                expected.len(),
                if !extra.is_empty() {
                    format!(", extra in obstacles: {:?}", extra)
                } else {
                    String::new()
                },
                if !missing.is_empty() {
                    format!(", missing in obstacles: {:?}", missing)
                } else {
                    String::new()
                },
            )
        } else {
            "PASS".to_string()
        }
    );

    
    println!("Time taken: {:?}", start.elapsed());

    Ok(())
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

fn check_right_for_loop(room: &mut [Vec<RoomSpace>], position: (usize,usize), direction: &Direction) -> Option<(usize,usize)> {
    if let Some((obsx,obsy)) = get_newspace(room, position, direction) {
        if room[obsx][obsy] == RoomSpace::Obstacle {
            return None;
        }
        room[position.0][position.1] = RoomSpace::Guard(direction.clone());
        room[obsx][obsy] = RoomSpace::Obstacle;
        println!("({},{})",obsx,obsy);
        let mut continue_moving = true;
        let mut checkpoints = Vec::new();
        let mut checktrail = Vec::new();
        while continue_moving {
            continue_moving = move_guard(room, &mut checktrail);
            //print_room(room, 500);
            println!("({},{})",obsx,obsy);
            if continue_moving && checkpoints.contains(checktrail.last().unwrap()) {
                println!("LOOP! {:?} obs: {} {}", checktrail.last().unwrap(),obsx,obsy);
                return Some((obsx,obsy))
            }
            if continue_moving {
                checkpoints.push(checktrail.last().unwrap().clone());
            }
        }
    };
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
        if dir == direction {
            room[newspace.0][newspace.1] = RoomSpace::Guard(dir.clone());
        } else {
            room[guard_pos.0][guard_pos.1] = RoomSpace::Guard(dir.clone());
        }
        true
    } else {
        false
    }
}
