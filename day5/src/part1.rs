use std::fs::File;
use std::time::Instant;
use std::io::{self, BufRead, BufReader};

pub fn run() -> io::Result<()> {
    let start = Instant::now();
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut rules = Vec::new();
    let mut updatepages:Vec<Vec<u32>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.contains("|") {
            let (pre,post) = line.split_once("|").unwrap();
            rules.push((pre.parse::<u32>().unwrap(),post.parse::<u32>().unwrap()));
        }
        if line.contains(",") {
            updatepages.push(line.split(",").map(|x|x.parse::<u32>().unwrap()).collect());
        }
    }

    for update in updatepages {
        if matches_rules(&rules, &update) {
        }
    }

    //println!("rules: {:#?}", rules);
    //println!("updatepages: {:#?}", updatepages);

    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}

fn matches_rules(rules: &[(u32, u32)], update: &[u32]) -> bool {
}
