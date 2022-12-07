use std::collections::{HashMap, HashSet};

use crate::{Solution, SolutionPair, etc::utils};

///////////////////////////////////////////////////////////////////////////////

pub fn solve<'a>() -> SolutionPair {

    let lines = utils::read_lines("./input/input_07");

    let mut dirSizes: HashMap<String, u64> = HashMap::new();
    let mut dirStack: Vec<String> = vec![];

    for line in &lines {
        if line.starts_with("$ cd") {
            if line.contains("..") {
                dirStack.pop();
            } else {
                dirStack.push(line.split(" ").last().unwrap().to_string());
                let dir = dirStack.iter().cloned().collect::<String>();
                if !dirSizes.contains_key(&dir) {
                    dirSizes.insert(dir.clone(), 0);
                }
            }
        } else if line.chars().next().unwrap().is_numeric() {
            let size = line.split(" ").next().unwrap().parse::<u64>().unwrap();
            for i in 0..dirStack.len() {
                let d = dirStack[..=i].iter().cloned().collect::<String>();
                *dirSizes.get_mut(&d).unwrap() += size;
            }
        }
    }
    
    // Your solution here...
    let sol1: u64 = dirSizes.clone().into_values().filter(|v| v <= &100000).sum();
    
    let toFree = dirSizes[&"/".to_string()] - 40000000;
    let smallest = dirSizes.into_values().filter(|v| v >= &toFree).min().unwrap();
    let sol2: u64 = smallest;

    (Solution::U64(sol1), Solution::U64(sol2))
}
