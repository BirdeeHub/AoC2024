use std::fs::File;
use std::time::Instant;
use std::io::{self, Read};
use std::env;

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

    let mut disk: Vec<Option<u64>> = Vec::new();
    let input: Vec<u64> = read_file(&filepath)?.trim().chars().map(|c| c.to_digit(10).unwrap() as u64).collect();
    let mut segment_num = 0;
    let mut i = 0;
    while i < input.len() {
        if let Some(num) = input.get(i) {
            if *num > 0 {
                for _ in 0..*num {
                    disk.push(Some(segment_num));
                }
                segment_num += 1;
            }
            if let Some(spaces) = input.get(i+1) {
                for _ in 0..*spaces {
                    disk.push(None);
                }
            }
        }
        i += 2;
    }
    while disk.last().is_none() {
        disk.pop();
    }

    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}
