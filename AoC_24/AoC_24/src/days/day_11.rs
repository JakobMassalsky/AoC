use std::iter;

use itertools::Itertools;

use crate::{etc::utils::{self, ParseAll}, Solution, SolutionPair};
use cached::proc_macro::cached;

///////////////////////////////////////////////////////////////////////////////

fn split_number(num: i64, depth: u64, base: i64) -> usize {
    blink(num/base, depth) + blink(num % base, depth)
}

#[cached]
fn blink(num: i64, depth: u64) -> usize {
    if depth == 0 { return 1 }
    let rem = depth - 1;
    
    if num == 0 { blink(1, rem) } 
    else {
        let digits = num.ilog10() + 1;
        if digits % 2 == 0 { split_number(num, rem, 10_i64.pow(digits/2)) } 
        else { blink(num * 2024, rem) }
    }
}

fn stare(seq: Vec<i64>, blinks: u64) -> u64 {
    let mut sum = 0;
    for d in seq {
        sum += blink(d, blinks)
        // println!("{:?}", i);
    }
    sum as u64
}

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_11")[0].split(" ").parse_all().collect_vec();

    // Your solution here...
    let sol1: u64 = stare(lines.clone(), 25);
    let sol2: u64 = stare(lines, 75);

    (Solution::U64(sol1), Solution::U64(sol2))
}
