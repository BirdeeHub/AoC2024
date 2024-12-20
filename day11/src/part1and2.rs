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
    let mut stones: Vec<u64> = read_file(&filepath)?.split_whitespace().map(|v| v.parse::<u64>().unwrap()).collect();

    for _ in 0..25 {
        stones = do_blink(&stones);
    }
    println!("Part 1, 25 blinks: {}", stones.len());

    for i in 0..50 {
        println!("Blink {}", i+26);
        stones = do_blink(&stones);
    }
    println!("Part 2, 75 blinks:: {}", stones.len());

    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}

fn do_blink(stones: &[u64]) -> Vec<u64> {
    stones.iter().fold(Vec::new(), |mut acc,v|{
        if *v == 0 {
            acc.push(1);
        } else {
            let numlen = ((*v as f64).log10().floor() as u64 + 1) as usize;
            if numlen % 2 == 0 {
                let charnum = v.to_string();
                let (st1, st2) = charnum.split_at(numlen/2);
                acc.push(st1.parse::<u64>().unwrap());
                acc.push(st2.parse::<u64>().unwrap());
            } else {
                acc.push(*v*2024);
            }
        }
        acc
    })
}
