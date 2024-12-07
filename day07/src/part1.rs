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

    let mut calibration_total = 0;
    for (answer, values) in equations {
        if check_equation(answer, values) {
            calibration_total += answer;
        }
    }

    println!("Calibration total: {}", calibration_total);
    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}

fn check_equation(answer: i64, values: Vec<i64>) -> bool {
    println!("{:?} = {:?}", answer, values);
    true
}
