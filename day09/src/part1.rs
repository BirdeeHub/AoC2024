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
    let input: Vec<u8> = read_file(&filepath)?.trim().chars().map(|c| c.to_digit(10).unwrap() as u8).collect();

    println!("{:?}",input);

    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}
