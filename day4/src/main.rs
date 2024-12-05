use std::fs::File;
use std::time::Instant;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let start = Instant::now();
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut wordsearch:Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        wordsearch.push(line.chars().collect());
    }

    let rows = wordsearch.len();
    let cols = wordsearch[0].len();
    let mut xmas_count = 0;

    println!("total XMAS: {}", xmas_count);
    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}
