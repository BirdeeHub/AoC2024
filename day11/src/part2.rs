use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::time::Instant;

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
        _ => env::var("AOC_INPUT").expect("AOC_INPUT not set"),
    };
    let mut stones: Vec<u64> = read_file(&filepath)?
        .split_whitespace()
        .map(|v| v.parse::<u64>().unwrap())
        .collect();

    for _ in 0..75 {
        do_blink(&mut stones);
    }
    println!("Part 2, 75 blinks: {}", stones.len());

    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}

fn do_blink(stones: &mut Vec<u64>) {
    let fulllen = stones.len();
    for i in 0..fulllen {
        let v = stones[i];
        if v == 0 {
            stones[i] = 1;
        } else {
            let numlen = (v as f64).log10().floor() as u64 + 1;
            if numlen % 2 == 0 {
                let divisor = 10u64.pow((numlen / 2) as u32);
                stones[i] = v / divisor;
                stones.push(v % divisor);
            } else {
                stones[i] = v * 2024;
            };
        };
    };
}
