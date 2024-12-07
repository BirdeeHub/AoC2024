use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone)]
pub enum RoomSpace {
    Guard(Direction),
    Obstacle,
    Visited,
    Empty,
}

impl Display for RoomSpace {
    fn fmt(&self, fmt:&mut Formatter) -> Result<(), std::fmt::Error> {
        fmt.write_str(match self {
            RoomSpace::Guard(dir) => match dir {
                Direction::Up => "^",
                Direction::Down => "v",
                Direction::Left => "<",
                Direction::Right => ">",
            },
            RoomSpace::Obstacle => "#",
            RoomSpace::Visited => ".",
            RoomSpace::Empty => " ",
        })
    }
}
use std::time::Duration;
use std::thread;
pub fn print_room(room: &[Vec<RoomSpace>], delay:u64) {
    thread::sleep(Duration::from_millis(delay));
    println!("{}","-".repeat(room[0].len()));
    if room.is_empty() {
        return;
    }

    let num_cols = room.len();
    let num_rows = room[0].len();

    for col in 0..num_rows {
        let row: String = (0..num_cols)
            .map(|row| room[row][col].to_string())
            .collect();
        println!("{}", row);
    }
    println!("{}","-".repeat(room[0].len()));
}

