use std::fs::File;
use std::time::Instant;
use std::collections::HashMap;
use std::io::{self, BufRead, BufReader};
use std::env;

fn main() -> io::Result<()> {
    let start = Instant::now();
    let inputvar = env::var("AOC_INPUT").expect("AOC_INPUT not set");
    let file = File::open(inputvar)?;
    let reader = BufReader::new(file);

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let (lhs, rhs) = line.split_once("   ").unwrap();
        left.push(lhs.parse::<i32>().unwrap());
        right.push(rhs.parse::<i32>().unwrap());
    }

    println!("{}",calc(left, right));
    println!("Time taken: {:?}", start.elapsed());
    
    Ok(())
}

fn calc(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut list1 = left;
    let mut list2 = right;
    list1.sort();
    list2.sort();
    let mut cache = HashMap::<i32, i32>::new();
    list1.into_iter().map(|v| {
        match cache.get(&v) {
            Some(val) => *val,
            None => {
                let filtered = list2.iter().filter(|val|{
                    **val == v
                }).count();
                let res: i32 = filtered as i32;
                cache.insert(v, res);
                res * v
            },
        }
    }).sum::<i32>()
}
