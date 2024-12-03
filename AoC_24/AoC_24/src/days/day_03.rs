use crate::{etc::utils::{self, SumBy}, Solution, SolutionPair};
use regex::Regex;

///////////////////////////////////////////////////////////////////////////////

fn part1(lines: String) -> u64 {

    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let matches: Vec<&str> = re.captures_iter(lines.as_str()).map(|c| c.extract::<0>().0).collect();
    // for m in matches {
    //     println!("{}", m)
    // }
    // 0
    matches.iter().map(|s| s.split(",")
        .map(|s| s.chars().filter(|c| c.is_numeric()).collect::<String>().parse::<u64>().unwrap()).collect::<Vec<u64>>())
        .map(|v| v[0]*v[1])
        .sum()
}

fn part2(lines: String) -> u64 {

    let re = Regex::new(r"do\(\)|don't\(\)|mul\(\d{1,3},\d{1,3}\)").unwrap();
    let matches: Vec<&str> = re.captures_iter(lines.as_str()).map(|c| c.extract::<0>().0).collect();
    // for m in matches {
    //     println!("{}", m)
    // }
    // 0
    let mut en: bool = true;
    let mut sum: u64 = 0;
    for &m in matches.iter() {
        if m.starts_with("do(") {en = true};
        if m.starts_with("don") {en = false};
        if m.starts_with("mul") && en {
            sum += m.split(",").map(|d| d.chars().filter(|c| c.is_numeric()).collect::<String>().parse::<u64>().unwrap()).product::<u64>();
        };
    }
    sum
}

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_03").into_iter().collect::<String>();

    // Your solution here...
    let sol1: u64 = part1(lines.clone());
    let sol2: u64 = part2(lines);

    (Solution::U64(sol1), Solution::U64(sol2))
}
