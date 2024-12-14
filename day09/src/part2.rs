use std::fs::File;
use std::time::Instant;
use std::io::{self, Read};
use std::env;

fn read_file(file_path: &str) -> io::Result<String> {
    let mut contents = String::new();
    File::open(file_path)?.read_to_string(&mut contents)?;
    Ok(contents)
}

#[derive(Debug, Clone, Copy)]
enum Segment {
    Block(usize,usize),
    Empty(usize),
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
            disk.push(Segment::Block(segment_num,*num));
            segment_num += 1;
            if let Some(spaces) = input.get(i+1) {
                if *spaces > 0 {
                    disk.push(Segment::Empty(*spaces));
                }
            }
        }
        i += 2;
    }
    while let Some(Segment::Empty(_)) = disk.last() {
        disk.pop();
    }
    println!("{:?}",disk);
    let mut revdisk: Vec<Segment> = disk.clone();
    revdisk.reverse();
    for (i, segment) in revdisk.iter().enumerate() {
        let mut to_add: Vec<(usize,Segment)> = Vec::new();
        match segment {
            Segment::Empty(_) => {
                continue;
            },
            Segment::Block(num,size) => {
                for (idx, val) in disk.iter_mut().enumerate() {
                    if idx >= revdisk.len()-1-i { break; };
                    if let Segment::Empty(width) = val {
                        match width {
                            _ if *width > *size => {
                                *val = Segment::Empty(*width-size);
                                to_add.push((idx,Segment::Block(*num,*size)));
                                break;
                            },
                            _ if *width == *size => {
                                *val = Segment::Block(*num,*size);
                                break;
                            },
                            _ => { continue; },
                        }
                    }
                }
            }
        }
        for (idx,block) in to_add.iter().rev() {
            disk.insert(*idx, *block);
        }
    }
    //TODO:
    // You need to pop the old ones off of disk after moving them

    let mut xpanddisk: Vec<Option<usize>> = Vec::new();
    for segment in disk.iter() {
        match segment {
            Segment::Block(num,size) => {
                for _ in 0..*size {
                    xpanddisk.push(Some(*num));
                }
            },
            Segment::Empty(size) => {
                for _ in 0..*size {
                    xpanddisk.push(None);
                }
            },
        }
    }

    println!("{:?}",xpanddisk);

    let mut checksum: u64 = 0;
    for (i, val) in xpanddisk.iter().enumerate() {
        if let Some(num) = val {
            checksum += *num as u64 * (i as u64);
        }
    }

    println!("{:?}",checksum);

    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}
