use std::collections::HashSet;
use itertools::Itertools;
use crate::{Solution, SolutionPair, etc::utils};

///////////////////////////////////////////////////////////////////////////////

fn get_sol(chars: &Vec<char>, n: usize) -> u64 {
    for (i, c) in chars.windows(n).enumerate() {
        if c.iter().all_unique() {
            return (i+n) as u64;
        }
    }
    return 0;
}

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_06");
    let chars = lines[0].chars().collect::<Vec<char>>();

    let sol1 = get_sol(&chars, 4);
    let sol2 = get_sol(&chars, 14);

    (Solution::U64(sol1), Solution::U64(sol2))
}
