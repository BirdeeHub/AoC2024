use std::fs::File;
use std::time::Instant;
use std::io::{self, BufRead, BufReader};

pub fn run() -> io::Result<()> {
    let start = Instant::now();
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut board:Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        board.push(line.chars().collect());
    }

    let rows = board.len();
    let cols = board[0].len();
    let target = ('M' as u32) + ('S' as u32);
    let mut xmas_count = 0;
    for x in 1..rows-1 {
        for y in 1..cols-1 {
            if board[x][y] == 'A' && (board[x-1][y-1] as u32) + (board[x+1][y+1] as u32) == target && (board[x+1][y-1] as u32) + (board[x-1][y+1] as u32) == target {
                xmas_count += 1;
            }
        }
    }

    println!("total XMAS: {}", xmas_count);
    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}
