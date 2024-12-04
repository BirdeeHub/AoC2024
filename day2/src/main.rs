use std::fs::File;
use std::time::Instant;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let start = Instant::now();
    let reader = BufReader::new(File::open("input")?);
    let mut results = Vec::<bool>::new();
    for line in reader.lines() {
        results.push(calc_with_dampener(
            &mut line?
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        ));
    }
    println!("{}", results.iter().filter(|x| **x).count());
    println!("Time taken: {:?}", start.elapsed());
    Ok(())
}

// we cant deal with if the first val is the bad one.
// So we try it forwards and backwards and if 1 is true, then we're good
fn calc_with_dampener(levels: &mut [i32]) -> bool {
    if calc(levels) { return true; }
    levels.reverse();
    if calc(levels) { return true; };
    false
}

fn calc(levels: &[i32]) -> bool {
    let mut last = 0;
    let mut last_diff = 0;
    let mut res = true;
    let mut chance = true;
    for (idx, level) in levels.iter().enumerate() {
        if idx == 0 {
            last = *level;
        } else {
            if !res { break; };
            let diff = level - last;
            match diff {
                _ if (last_diff > 0 && diff < 0) || (last_diff < 0 && diff > 0) => {
                    res = false;
                },
                _ if diff.abs() < 1 || diff.abs() > 3 => {
                    res = false;
                },
                _ => {},
            };
            if chance && !res {
                res = true;
                chance = false;
            } else {
                last = *level;
                last_diff = diff;
            }
        };
    };
    res
}
