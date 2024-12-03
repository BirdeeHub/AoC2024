use std::fs::File;
use std::collections::HashMap;
use std::path::Path;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open(Path::new("input"))?;
    let reader = BufReader::new(file);

    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut list = 0;
        for word in line.split_whitespace() {
            if list == 0 {
                list = 1;
                left.push(word.parse::<i64>().unwrap());
            } else if list == 1 {
                list = 0;
                right.push(word.parse::<i64>().unwrap());
            }
        }
    }

    println!("{:?}",calc(left, right));
    
    Ok(())
}

fn calc(left: Vec<i64>, right: Vec<i64>) -> i64 {
    let mut list1 = left;
    let mut list2 = right;
    list1.sort();
    list2.sort();
    let mut cache = HashMap::<i64, i64>::new();
    list1.into_iter().map(|v| {
        match cache.get(&v) {
            Some(val) => *val,
            None => {
                let filtered = list2.iter().filter(|val|{
                    **val == v
                }).count();
                let res: i64 = filtered as i64;
                cache.insert(v, res);
                res * v
            },
        }
    }).sum::<i64>()
}
