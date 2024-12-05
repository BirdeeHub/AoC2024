use std::fs::File;
use std::collections::{HashMap, VecDeque};
use std::time::Instant;
use std::io::{self, BufRead, BufReader};
use std::env;

pub fn run() -> io::Result<()> {
    let start = Instant::now();
    let inputvar = env::var("AOC_INPUT").expect("AOC_INPUT not set");
    let file = File::open(inputvar)?;
    let reader = BufReader::new(file);

    let mut updatepages:Vec<Vec<u32>> = Vec::new();
    let mut graph:HashMap<u32,Vec<u32>> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        if line.contains("|") {
            let (pre,post) = line.split_once("|").unwrap();
            graph.entry(pre.parse::<u32>().unwrap()).or_default().push(post.parse::<u32>().unwrap());
        }
        if line.contains(",") {
            updatepages.push(line.split(",").map(|x|x.parse::<u32>().unwrap()).collect());
        }
    }
    for posts in graph.values_mut() {
        posts.dedup();
    }

    let mut mids_p1 = Vec::new();
    let mut mids_p2 = Vec::new();

    for update in updatepages {
        let fixed = toposort_update(&graph, &update);
        if update == fixed {
            let middleidx = (fixed.len()-1)/2;
            mids_p1.push(fixed[middleidx]);
        } else {
            let middleidx = (fixed.len()-1)/2;
            mids_p2.push(fixed[middleidx]);
        };
    }

    println!("Part 1: {}", mids_p1.iter().sum::<u32>());
    println!("Part 2: {}", mids_p2.iter().sum::<u32>());

    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}

fn toposort_update(rules: &HashMap<u32, Vec<u32>>, update: &[u32]) -> Vec<u32> {
    let mut final_update = Vec::new();

    // Step 1: Compute in-degrees for nodes in update
    let mut in_degree = HashMap::new();
    for &page in update {
        in_degree.entry(page).or_insert(0); // Initialize all nodes with 0 in-degree
    }
    for (key, posts) in rules {
        if update.contains(key) {
            for post in posts {
                if update.contains(post) {
                    *in_degree.entry(*post).or_insert(0) += 1;
                }
            }
        }
    }

    // Step 2: Collect nodes with in-degree 0 into a queue
    let mut queue = VecDeque::new();
    for &page in update {
        if let Some(&0) = in_degree.get(&page) {
            queue.push_back(page);
        }
    }

    // Step 3: Perform topological sorting
    while let Some(current) = queue.pop_front() {
        final_update.push(current);

        // after pushing the next in queue, reduce the in-degree of its dependents
        // then add the next ones, with in-degree 0 to the queue
        if let Some(dependents) = rules.get(&current) {
            for &dependent in dependents {
                if let Some(in_deg) = in_degree.get_mut(&dependent) {
                    *in_deg -= 1;
                    if *in_deg == 0 {
                        queue.push_back(dependent);
                    }
                }
            }
        }
    }

    //// Step 4: Handle any remaining pages
    for &page in update {
        if !final_update.contains(&page) {
            final_update.push(page);
        }
    }

    final_update
}
