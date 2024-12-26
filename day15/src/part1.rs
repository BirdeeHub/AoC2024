use std::fs::File;
use std::time::Instant;
use std::io::{self, Read};
use std::env;

#[derive(Debug,Clone,Copy)]
enum Move {
    L,
    R,
    U,
    D,
}
#[derive(Debug,Clone,Copy)]
enum Space {
    Empty,
    Wall,
    Box,
    Robot,
}

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
    let mapstr = inparts[0];
    let mut map:Vec<Vec<Space>> = Vec::new();
    for line in mapstr.lines() {
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
        map.push(row);
    }
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

    for row in map {
        println!("{:?}",row)
    }
    println!("Moves: {:?}", moves);

    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}
