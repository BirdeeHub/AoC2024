use std::fs::File;
use std::time::Instant;
use std::io::{self, BufRead, BufReader};
use std::env;

pub fn run() -> io::Result<()> {
    let start = Instant::now();
    let inputvar = env::var("AOC_INPUT").expect("AOC_INPUT not set");
    let file = File::open(inputvar)?;
    let reader = BufReader::new(file);

    let mut equations:Vec<(i64, Vec<i64>)> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let (ans, rest) = line.split_once(":").unwrap();
        let values = rest.split_whitespace().map(|v|v.parse::<i64>().unwrap()).collect();
        equations.push((ans.parse::<i64>().unwrap(), values));
    }

    for equation in equations {
        println!("{:?}", equation);
    }

    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}
