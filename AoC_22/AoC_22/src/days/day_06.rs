use std::collections::HashSet;
use itertools::Itertools;
use crate::{Solution, SolutionPair, etc::utils};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_06");

    let mut rb = vec!['0', '0', '0', '0'];
    let mut rb2 = vec!['0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0'];
    let mut sol1: u64 = 0;
    let mut sol2: u64 = 0;

    for (i, c) in lines[0].char_indices() {
        rb[i%4] = c;
        rb2[i%14] = c;
        if rb.iter().unique().count() == 4 && i >= 4 && sol1 == 0 { 
            sol1 = i as u64 + 1;
        }
        if rb2.iter().unique().count() == 14 && i >= 14 && sol2 == 0 {
            sol2 = i as u64 + 1;
            break;
        }
    }


    (Solution::U64(sol1), Solution::U64(sol2))
}
