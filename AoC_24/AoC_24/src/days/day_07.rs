use itertools::{Combinations, Itertools};

use crate::{etc::utils::{self, ParseAll, SumBy}, Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

// bruh gpt
fn combine_numbers(a: u64, b: u64) -> u64 {
    let mut multiplier = 1;
    // let mut temp = b;

    // Calculate the power of 10 that matches the number of digits in `b`
    while multiplier <= b {
        multiplier *= 10;
    }

    // Combine `a` and `b`
    a * multiplier + b
}

fn part1(line: &Vec<u64>) -> u64 {
    let res = line[0];
    
    if (0..1<<(line.len()-1)).into_iter().any(|i| {
        line[2..].iter().enumerate().fold(line[1], |l, (id, &r)| if ((1 << id) & i) > 0  {l+r} else {l*r}) == res //((1 << id) & i) > 0 
    }) {
        res
    } else {
        0
    }
}

fn part2(line: &Vec<u64>, combinations: &Vec<Vec<usize>>) -> u64 {
    let res = line[0];

    if combinations.iter().take(3_usize.pow((line.len()-1) as u32)).any(|i| {
        line[2..].iter().enumerate().fold(line[1], |l, (id, &r)| match i[id] {
            0 => l+r,
            1 => l*r,
            _ => combine_numbers(l, r),
        }) == res
    }) {
        res
    } else {
        0
    }
}

pub fn solve() -> SolutionPair {

    let lines: Vec<Vec<u64>> = utils::read_lines("./input/input_07").iter()
        .map(|s| s.replace(":", "").split(" ").parse_all().collect()).collect();

    let length = lines.iter().map(|v| v.len()).max().unwrap() - 2; // Replace with `line.len() - 1` or the desired length
    let num_combinations = 3_usize.pow(length as u32);

    let mut v = Vec::new();

    // le gpt
    for i in 0..num_combinations {
        let mut sequence = Vec::new();
        let mut value = i;
        for _ in 0..length {
            sequence.push(value % 3); // Get the last digit in base 3
            value /= 3; // Move to the next digit
        }
        v.push(sequence);
    } 
    // Your solution here...
    let sol1: u64 = lines.iter().sum_by(|l| part1(l));
    let sol2: u64 = lines.iter().filter(|l| part1(l) <= 0).sum_by(|l| part2(l, &v)) + sol1;    
 
    (Solution::U64(sol1), Solution::U64(sol2))
}
