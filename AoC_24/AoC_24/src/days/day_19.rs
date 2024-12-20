use itertools::Itertools;
use regex::Regex;

use crate::{etc::utils::{self, SumBy}, Solution, SolutionPair};
use cached::proc_macro::cached;

///////////////////////////////////////////////////////////////////////////////

fn test_pattern(towels: &Regex, pattern: &String) -> bool {
    return towels.is_match(pattern)
}

#[cached]
fn count_pattern(towels: String, pattern: String) -> u64 {
    if pattern.len() == 0 {return 1}
    let mut acc = 0;
    for t in towels.split(", ") {
        if pattern.starts_with(t) {
            acc += count_pattern(towels.clone(), pattern[t.len()..].to_string())
        }
    }
    acc
}

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_19");

    let towels: Vec<&str> = lines[0].split(", ").collect();
    let re = Regex::new(("^(".to_owned() + towels.join("|").as_str() + ")+$").as_str()).unwrap();
    let patterns: Vec<&String> = lines[2..].into_iter().collect();
    
    // Your solution here...
    let sol1: u64 = patterns.iter().filter(|&p| test_pattern(&re, p)).count() as u64;
    let sol2: u64 = patterns.into_iter().sum_by(|p| count_pattern(lines[0].clone(), p.to_string())) as u64;

    (Solution::U64(sol1), Solution::U64(sol2))
}
