use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // Open the file
    let file = File::open(Path::new("input"))?;
    
    // Create a buffered reader
    let reader = BufReader::new(file);

    let mut results = Vec::<bool>::new();

    // Iterate over the lines in the file
    for line in reader.lines() {
        // Handle each line
        let line = line?; // Unwrap the Result to get the line
        let mut levels = Vec::<i32>::new();
        // Split the line into words (by whitespace)
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
            last = *level;
            last_diff = diff;
        };
    };
    res
}

/// Check if levels are safe with the Problem Dampener
fn calc_with_dampener(levels: &[i32]) -> bool {
    if calc(levels) {
        return true; // Already safe
    }

    // Try removing each level to see if it becomes safe
    for i in 0..levels.len() {
        let mut reduced_levels = levels.to_vec();
        reduced_levels.remove(i);
        if calc(&reduced_levels) {
            return true; // Safe after removing one level
        }
    }

    false // Unsafe even with the dampener
}
