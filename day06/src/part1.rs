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

    let mut continue_moving = true;
    while continue_moving {
        continue_moving = move_guard(&mut room);
        //room.print(250)
    }
    
    let visited = room.iter().flat_map(|row| row.iter()).filter(|&cell| cell == &RoomSpace::Visited).count();

    println!("total visited: {}", visited);

    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}

fn move_guard(room: &mut [Vec<RoomSpace>]) -> bool {
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
    let get_newspace = |dir| {
        match dir {
            Direction::Up => {
                if guard_pos.1 > 0 { Some((guard_pos.0, guard_pos.1 - 1)) } else { None }
            },
            Direction::Down => {
                if guard_pos.1 + 1 < room[guard_pos.0].len() { Some((guard_pos.0, guard_pos.1 + 1)) } else { None }
            },
            Direction::Right => {
                if guard_pos.0 + 1 < room.len() { Some((guard_pos.0 + 1, guard_pos.1)) } else { None }
            },
            Direction::Left => {
                if guard_pos.0 > 0 { Some((guard_pos.0 - 1, guard_pos.1)) } else { None }
            },
        }
    };
    if let Some(newspace) = get_newspace(direction.clone()) {
        let newdirection = match room[newspace.0][newspace.1] {
            RoomSpace::Obstacle => {
                match direction {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                }
            },
            _ => direction.clone(),
        };
        if direction == newdirection {
            room[newspace.0][newspace.1] = RoomSpace::Guard(newdirection);
            true
        } else if let Some(newplace) = get_newspace(newdirection.clone()) {
            room[newplace.0][newplace.1] = RoomSpace::Guard(newdirection);
            true
        } else {
            false
        }
    } else {
        false
    }
}
