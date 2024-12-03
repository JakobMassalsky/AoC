use itertools::Itertools;

use crate::{etc::utils::{self, ParseAll, SkipRange}, Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

fn check(p: Vec<i64>) -> bool {
    p.iter().all(|&x| x >= -3 && x < 0) || p.iter().all(|&x| x > 0 && x <= 3)
}

fn check_line(l: &&String) -> bool {
    let p = l.split(" ").parse_all::<i64>().collect::<Vec<i64>>();
    (0..p.len()).any(|i|
        check(p.iter().skip_range(i, 1)
            .tuple_windows().map(|(a, b)| b-a).collect::<Vec<i64>>()))
}

fn part1(lines: Vec<String>) -> u64 {
    lines.iter().filter(check_line).count() as u64
}

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_02");

    // Your solution here...
    let sol1: u64 = part1(lines);
    let sol2: u64 = 0;

    (Solution::U64(sol1), Solution::U64(sol2))
}
