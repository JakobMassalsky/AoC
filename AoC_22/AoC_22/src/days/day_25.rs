use itertools::Itertools;

use crate::{Solution, SolutionPair, etc::utils};

///////////////////////////////////////////////////////////////////////////////

fn snafu_to_digit(c: char) -> i32 {
    match c {
        '=' => -2,
        '-' => -1,
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => unreachable!()
    }
}

fn digit_to_snafu(d: &i32) -> char {
    match d {
        -2 => '=',
        -1 => '-',
        0  => '0',
        1  => '1',
        2  => '2',
        _  => unreachable!()
    }
}

fn add_snafu(left: &String, right: &String) -> String {
    let mut acc = vec![];
    let mut carriage = 0;
    let left = left.chars().rev().collect_vec();
    let right = right.chars().rev().collect_vec();
    for i in 0..left.len().max(right.len()) {
        let l = if i < left.len() { snafu_to_digit(left[i]) } else { 0 };
        let r = if i < right.len() { snafu_to_digit(right[i]) } else { 0 };
        let mut sum = l + r + carriage;
        if sum > 2 {
            sum -= 5;
            carriage = 1;
        } else if sum < -2 {
            sum += 5;
            carriage = -1;
        } else {
            carriage = 0;
        }
        acc.push(sum);
    }
    if carriage != 0 { acc.push(carriage) }
    acc.iter().map(digit_to_snafu).rev().collect::<String>()
}

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_25");

    // Your solution here...
    let sol1: String = lines.iter().cloned().reduce(|l, r| add_snafu(&l, &r)).unwrap();
    let sol2: u64 = 0;

    (Solution::Str(sol1), Solution::U64(sol2))
}
