use std::fs::File;
use std::time::Instant;
use std::io::{self, BufRead, BufReader};
use std::env;

pub fn run() -> io::Result<()> {
    let start = Instant::now();
    let inputvar = env::var("AOC_INPUT").expect("AOC_INPUT not set");
    let file = File::open(inputvar)?;
    let reader = BufReader::new(file);

    let mut board:Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        board.push(line.chars().collect());
    }

    let mut xmas_count = 0;

    for x in 0..board.len() {
        for y in 0..board[0].len() {
            if board[x][y] == 'X' {
                xmas_count += calc(&board, x as i32, y as i32);
            }
        }
    }

    println!("part1 try2 total XMAS: {}", xmas_count);
    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}
fn check_direction(board: &[Vec<char>], rows: i32, cols: i32, x: i32, y: i32, dx: i32, dy: i32) -> u32 {
    let mut i = x;
    let mut j = y;
    let mut idx = 0;
    let target = ['X','M','A','S'];
    while idx<4 && i>=0 && j>=0 && i<rows && j<cols {
        if target[idx] != board[i as usize][j as usize] {
            return 0;
        }
        i += dx;
        j += dy;
        idx += 1;
    }
    if idx==4 {1} else {0}
}
fn calc(board: &[Vec<char>], x: i32, y: i32) -> u32 {
    let directions = [
        (1,1),
        (-1,1),
        (1,-1),
        (-1,-1),
        (0,1),
        (1,0),
        (-1,0),
        (0,-1),
    ];
    let rows = board.len() as i32;
    let cols = board[0].len() as i32;
    let mut count = 0;
    for (dx,dy) in directions {
        count += check_direction(board, rows, cols, x, y, dx, dy)
    }
    count
}
