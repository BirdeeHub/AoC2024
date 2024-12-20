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

    let mut disk: Vec<Option<u64>> = Vec::new();
    let input: Vec<u64> = read_file(&filepath)?.trim().chars().map(|c| c.to_digit(10).unwrap() as u64).collect();
    let mut segment_num = 0;
    let mut i = 0;
    while i < input.len() {
        if let Some(num) = input.get(i) {
            if *num > 0 {
                for _ in 0..*num {
                    disk.push(Some(segment_num));
                }
                segment_num += 1;
            }
            if let Some(spaces) = input.get(i+1) {
                for _ in 0..*spaces {
                    disk.push(None);
                }
            }
        }
        i += 2;
    }

    let mut i = disk.len() - 1;

    while i > 0 {
        let val = disk[i];
        if let Some(num) = val {
            let mut len1 = 0;
            while i-len1 > 0 && disk[i-len1].is_some_and(|v| v == num) {
                len1 += 1;
            }
            let mut len2 = 0;
            let mut j = 0;
            while j+len2 < i {
                if len2 >= len1 {
                    for k in 0..len1 {
                        disk[j+k] = Some(num);
                        disk[i-k] = None;
                    }
                    break;
                } else if disk[j+len2].is_none() {
                    len2 += 1;
                } else {
                    j += len2 + 1;
                    len2 = 0;
                }
            }
            i -= len1;
        } else {
            i -= 1;
        }
    }

    let mut checksum: u64 = 0;
    for (i, val) in disk.iter().enumerate() {
        if let Some(num) = val {
            checksum += *num * (i as u64);
        }
    }

    println!("{:?}",checksum);

    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}
