use itertools::Itertools;

use crate::{etc::utils::{self, parse_eager, ParseAll, SumBy}, Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn part1(nums: &Vec<Vec<u64>>) -> u64 {
    nums.iter()
        .map(|s| s[0])
        .sorted()
        .zip(nums.iter().map(|s| s[1]).sorted())
        .sum_by(|(n1, n2)| (n1).abs_diff(n2))
}

pub fn part2(nums: &Vec<Vec<u64>>) -> u64 {
    let n2 = nums.iter().map(|s| s[1]).counts();
    nums.iter()
        .sum_by(|s| *n2.get(&s[0]).unwrap_or(&0) as u64 * s[0])
}

pub fn solve() -> SolutionPair {
    let lines: Vec<Vec<u64>> = utils::read_lines("./input/input_01")
        .iter().map(|s| s.split("   ").map(parse_eager).collect()).collect();

    // Your solution here...
    let sol1: u64 = part1(&lines);
    let sol2: u64 = part2(&lines);

    (Solution::U64(sol1), Solution::U64(sol2))
}
