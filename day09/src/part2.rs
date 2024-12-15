use std::fs::File;
use std::time::Instant;
use std::io::{self, Read};
use std::env;

fn read_file(file_path: &str) -> io::Result<String> {
    let mut contents = String::new();
    File::open(file_path)?.read_to_string(&mut contents)?;
    Ok(contents)
}

#[derive(Debug, Clone)]
enum Segment {
    Block(Vec<Option<usize>>),
    Empty(Vec<Option<usize>>),
}

pub fn run() -> io::Result<()> {
    let start = Instant::now();
    let args: Vec<String> = std::env::args().collect();
    let filepath = match args.get(1) {
        Some(fp) => fp.to_string(),
        _ => env::var("AOC_INPUT").expect("AOC_INPUT not set")
    };
    let mut disk: Vec<Segment> = Vec::new();
    let input: Vec<usize> = read_file(&filepath)?.trim().chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
    let mut segment_num = 0;
    let mut i = 0;
    while i < input.len() {
        if let Some(num) = input.get(i) {
            disk.push(Segment::Block(vec![Some(segment_num); *num]));
            segment_num += 1;
            if let Some(spaces) = input.get(i+1) {
                if *spaces > 0 {
                    disk.push(Segment::Empty(vec![None; *spaces]));
                }
            }
        }
        i += 2;
    }
    while let Some(Segment::Empty(_)) = disk.last() {
        disk.pop();
    }
    println!("{:?}",disk);
    for i in (disk.len()-1)..=0 {
        if let Some(Segment::Block(vals)) = disk.get_mut(i) {
            continue;
        };
        for val in &mut disk {
            if let Segment::Empty(vals2) = val {
            }
        }
    }

    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}
