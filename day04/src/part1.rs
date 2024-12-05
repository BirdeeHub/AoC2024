use std::fs::File;
use std::time::Instant;
use std::io::{self, BufRead, BufReader};
use std::env;

pub fn run() -> io::Result<()> {
    let start = Instant::now();
    let inputvar = env::var("AOC_INPUT").expect("AOC_INPUT not set");
    let file = File::open(inputvar)?;
    let reader = BufReader::new(file);

    let mut wordsearch:Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        wordsearch.push(line.chars().collect());
    }

    let mut xmas_count = 0;

    for _ in 0..4 {
        xmas_count += check_forward(&wordsearch);
        xmas_count += check_diag(&wordsearch);
        wordsearch = rotate(&wordsearch);
    }

    println!("total XMAS: {}", xmas_count);
    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}

fn rotate(board:&[Vec<char>]) -> Vec<Vec<char>> {
    let mut res = Vec::new();
    for i in 0..board[0].len() {
        let mut newrow = Vec::new();
        board.iter().for_each(|row|newrow.push(row[i]));
        res.push(newrow);
    };
    res.iter_mut().for_each(|row| row.reverse());
    res
}

fn check_forward(board: &[Vec<char>]) -> i32 {
    let mut count = 0;
    for row in board {
        let row_str: String = row.iter().collect();
        count += row_str.matches("XMAS").count() as i32;
    }
    count
}
fn check_diag(board: &[Vec<char>]) -> i32 {
    let mut count = 0;
    let rows = board.len();
    let cols = board[0].len();

    // Check diagonals starting from each row in the first column
    for row_start in 0..rows {
        let mut diag = String::new();
        let mut row = row_start;
        let mut col = 0;
        while row < rows && col < cols {
            diag.push(board[row][col]);
            row += 1;
            col += 1;
        }
        count += diag.matches("XMAS").count() as i32;
    }

    // Check diagonals starting from each column in the first row (except the first element)
    for col_start in 1..cols {
        let mut diag = String::new();
        let mut row = 0;
        let mut col = col_start;
        while row < rows && col < cols {
            diag.push(board[row][col]);
            row += 1;
            col += 1;
        }
        count += diag.matches("XMAS").count() as i32;
    }

    count
}
