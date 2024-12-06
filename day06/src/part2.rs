use std::fs::File;
use std::time::Instant;
use std::io::{self, BufRead, BufReader};
use std::env;

use crate::types::*;

pub fn run() -> io::Result<()> {
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
        (continue_moving, _) = move_guard(&mut room_with_guard, &mut trail);
        //print_room(&room)
    }

    let mut obstacles = Vec::new();
    for (dir, (x,y)) in trail.iter() {
        if let Some(obs) = check_right_for_loop(&mut guardless_room.clone(), &trail, (*x,*y), dir) {
            obstacles.push(obs);
        }
    }

    println!("locations: {:?}",obstacles);
    println!("number: {:?}",obstacles.len());
    
    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}

fn get_newspace(room: &[Vec<RoomSpace>], pos: (usize,usize), direction: &Direction) -> Option<(usize, usize)> {
    match direction {
        Direction::Up => {
            if pos.1 > 0 { Some((pos.0, pos.1 - 1)) } else { None }
        },
        Direction::Down => {
            if pos.1 + 1 < room[pos.0].len() { Some((pos.0, pos.1 + 1)) } else { None }
        },
        Direction::Right => {
            if pos.0 + 1 < room.len() { Some((pos.0 + 1, pos.1)) } else { None }
        },
        Direction::Left => {
            if pos.0 > 0 { Some((pos.0 - 1, pos.1)) } else { None }
        },
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

fn check_right_for_loop(room: &mut [Vec<RoomSpace>], trail: &[(Direction,(usize,usize))], position: (usize,usize), direction: &Direction) -> Option<(usize,usize)> {
    if let Some((obsx,obsy)) = get_newspace(room, position, direction) {
        room[position.0][position.1] = RoomSpace::Guard(direction.clone());
        room[obsx][obsy] = RoomSpace::Obstacle;
        let mut continue_moving = true;
        let mut checkpoint:Option<(Direction,(usize,usize))> = None;
        while continue_moving {
            let loc;
            (continue_moving, loc) = move_guard(room, &mut Vec::new());
            if let Some(location) = &checkpoint {
                if location == &loc {
                    println!("Found loop!");
                    println!("curr: {:?} check: {:?}", loc, location);
                    return Some((obsx,obsy))
                }
            }
            if continue_moving && trail.contains(&loc) && checkpoint.is_none() {
                checkpoint = Some(loc.clone());
            }
        }
    };
    None
}

fn move_guard(room: &mut [Vec<RoomSpace>], trail: &mut Vec<(Direction,(usize,usize))>) -> (bool,(Direction,(usize,usize))) {
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
        return (false, (direction,guard_pos))
    }
    room[guard_pos.0][guard_pos.1] = RoomSpace::Visited;
    if let Some(newspace) = get_newspace(room, guard_pos, &direction) {
        let newdirection = match room[newspace.0][newspace.1] {
            RoomSpace::Obstacle => turn_right(&direction),
            _ => direction.clone(),
        };
        if direction == newdirection {
            room[newspace.0][newspace.1] = RoomSpace::Guard(newdirection.clone());
            trail.push((newdirection.clone(),newspace));
            (true, (newdirection,guard_pos))
        } else if let Some(newplace) = get_newspace(room, guard_pos, &direction) {
            room[newplace.0][newplace.1] = RoomSpace::Guard(newdirection.clone());
            trail.push((newdirection.clone(),newplace));
            (true, (newdirection,guard_pos))
        } else {
            (false, (newdirection,guard_pos))
        }
    } else {
        (false, (direction,guard_pos))
    }
}
