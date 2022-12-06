use itertools::Itertools;
use crate::{Solution, SolutionPair, etc::utils};

///////////////////////////////////////////////////////////////////////////////

fn get_sol(chars: &Vec<char>, n: usize) -> usize {
    chars
        .windows(n)
        .position(|x| x.iter().all_unique())
        .unwrap() + n
}

pub fn solve() -> SolutionPair {

    let line = include_str!("../../input/input_06")
        .chars().collect::<Vec<char>>();
        
    let sol1 = get_sol(&line, 4) as u64;
    let sol2 = get_sol(&line, 14) as u64;

    (Solution::U64(sol1), Solution::U64(sol2))
}
