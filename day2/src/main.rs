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
        results.push(calc(levels));
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

fn calc(levels: Vec<i32>) -> bool {
    let mut last = 0;
    let mut last_diff = 0;
    let mut idx = 0;
    let mut res = true;
    for level in &levels {
        if idx == 0 {
            last = *level;
            idx += 1;
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
