use std::env;
use std::collections::HashMap;
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
    let stones: Vec<u64> = read_file(&filepath)?
        .split_whitespace()
        .map(|v| v.parse::<u64>().unwrap())
        .collect();

    let mut memo: HashMap<(u64,u64), u64> = HashMap::new();

    let res = stones.iter().map(|v| count(*v, 75, &mut memo)).sum::<u64>();

    println!("Part 2, 25 blinks: {}", res);

    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}

fn count(stone: u64, steps: u64, memo: &mut HashMap<(u64,u64), u64>) -> u64 {
    if let Some(&res) = memo.get(&(stone, steps)) {
        return res;
    }
    let ret;
    if steps == 0 {
        ret = 1;
    } else if stone == 0 {
        ret = count(1, steps - 1, memo);
    } else {
        let numlen = (stone as f64).log10().floor() as u64 + 1;
        if numlen % 2 == 0 {
            let divisor = 10u64.pow((numlen / 2) as u32);
            ret = count(stone / divisor, steps - 1, memo) + count(stone % divisor, steps - 1, memo);
        } else {
            ret = count(stone * 2024, steps - 1, memo);
        };
    }
    memo.insert((stone, steps), ret);
    ret
}
