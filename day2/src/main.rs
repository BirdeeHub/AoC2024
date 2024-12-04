use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open(Path::new("input"))?;
    let reader = BufReader::new(file);
    let mut results = Vec::<bool>::new();
    for line in reader.lines() {
        let line = line?;
        let mut levels = Vec::<i32>::new();
        for word in line.split_whitespace() {
            levels.push(word.parse::<i32>().unwrap());
        }
        results.push(calc_with_dampener(&mut levels));
    }

    println!("nice {}", results.iter().filter(|x| **x).count());

    Ok(())
}

fn calc_with_dampener(levels: &mut [i32]) -> bool {
    if calc(levels) {
        return true;
    }
    // we cant deal with if the first val is the bad one.
    // So we try it forwards and backwards and if 1 is true, then we're good
    levels.reverse();
    if calc(levels) {
        return true;
    };
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
            if ! res { break; };
            let diff = level - last;
            if last_diff > 0 && diff < 0 {
                res = false;
            }
            if last_diff < 0 && diff > 0 {
                res = false;
            }
            match diff {
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
