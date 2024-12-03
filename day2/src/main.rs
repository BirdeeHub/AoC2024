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
        results.push(calc_with_dampener(&levels));
    }

    let mut count = 0;
    let mut count2 = 0;

    for res in results {
        if res {
            count += 1;
        } else {
            count2 += 1;
        }
    };

    println!("nice {}", count);
    println!("naughty {}", count2);

    Ok(())
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

fn calc_with_dampener(levels: &[i32]) -> bool {
    if calc(levels) {
        return true;
    }
    // we cant deal with if the first val is the bad one.
    // So we try it forwards and backwards and if 1 is true, then we're good
    let mut reversed = levels.to_vec();
    reversed.reverse();
    if calc(&reversed) {
        return true;
    };
    false
}