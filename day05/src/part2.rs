use std::fs::File;
use std::time::Instant;
use std::io::{self, BufRead, BufReader};
use std::env;

pub fn run() -> io::Result<()> {
    let start = Instant::now();
    let inputvar = env::var("AOC_INPUT").expect("AOC_INPUT not set");
    let file = File::open(inputvar)?;
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

    let mut middles = Vec::new();

    for update in updatepages {
        if ! matches_rules(&rules, &update) {
            let fixed = reorder_update(&rules, &update);
            let middleidx = (fixed.len()-1)/2;
            middles.push(fixed[middleidx]);
        }
    }

    println!("Part 2 (first try): {}", middles.iter().sum::<u32>());

    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}

fn matches_rules(rules: &[(u32, u32)], update: &[u32]) -> bool {
    for (pre,post) in rules {
        let mut prefound = false;
        if update.contains(pre) && update.contains(post) {
            for page in update {
                if page == pre {
                    prefound = true;
                }
                if page == post && !prefound {
                    return false
                }
            }
        }
    }
    true
}

fn reorder_update(rules: &[(u32, u32)], update: &[u32]) -> Vec<u32> {
    let mut final_update = update.to_vec();

    while ! matches_rules(rules,&final_update) {
        for (pre,post) in rules {
            if final_update.contains(pre) && final_update.contains(post) {
                let prepos = final_update.iter().position(|x|x==pre).unwrap();
                let postpos = final_update.iter().position(|x|x==post).unwrap();
                if prepos > postpos {
                    final_update.remove(prepos);
                    final_update.insert(postpos,*pre);
                }
            }
        }
    }

    final_update
}
