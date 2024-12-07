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

    let operators = vec![
      Operator::new("+".to_string(), |a, b| { a + b }),
      Operator::new("*".to_string(), |a, b| { a * b })
    ];

    let mut calibration_total = 0;
    for (answer, values) in equations {
        if let Some(result) = find_expression(answer, &values, &operators) {
            println!("{} = {}", answer, result);
            calibration_total += answer;
        }
    }

    println!("Calibration total: {}", calibration_total);
    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}

#[derive(Debug, Clone)]
struct Operator {
    name: String,
    func: fn(i64, i64) -> i64,
}
impl Operator {
    fn new(name: String, func: fn(i64, i64) -> i64) -> Self {
        Operator { name, func }
    }
}

fn find_expression(target: i64, numbers: &[i64], operators: &[Operator]) -> Option<String> {
    let n = numbers.len();
    if n < 2 {
        return None; // Not enough numbers to form an expression
    }

    let num_operators = operators.len();
    let total_combinations = num_operators.pow((n - 1) as u32); // Number of operator combinations

    for i in 0..total_combinations {
        let mut expression = numbers[0].to_string();
        let mut value = numbers[0];
        let mut valid = true;
        let mut current_index = i;

        for j in 0..n - 1 {
            let operator_index = current_index % num_operators; // Extract the current operator index
            current_index /= num_operators; // Move to the next operator

            let operator = operators[operator_index].clone();
            let next_number = numbers[j + 1];

            expression.push_str(&operator.name);
            expression.push_str(&next_number.to_string());

            value = (operator.func)(value, next_number);

            // Early exit if value exceeds the target
            if value > target {
                valid = false;
                break;
            }
        }

        if valid && value == target {
            return Some(expression);
        }
    }

    None
}
