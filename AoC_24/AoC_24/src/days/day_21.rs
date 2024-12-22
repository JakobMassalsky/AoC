use std::{collections::HashMap, iter::{repeat_n, repeat_with}};

use cached::proc_macro::cached;
use itertools::Itertools;

use crate::{etc::utils::{self, SumBy}, Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

fn map_num(((x1, y1), (x2, y2)): ((usize, usize), (usize, usize)), bl: bool) -> String {
    let h = repeat_n(if x2 > x1 {'>'} else {'<'}, x1.abs_diff(x2)).collect::<String>();
    let v = repeat_n(if y2 > y1 {'v'} else {'^'}, y1.abs_diff(y2)).collect::<String>();
    (if (x2 == 0 && y1 == 3 && bl) || (x2 == 0 && y1 == 0 && !bl) {
        v + h.as_str()
    } else if (y2 == 3 && x1 == 0 && bl) || (x1 == 0 && y2 == 0 && !bl) {
        h + v.as_str()
    } else if x1 < x2 {
        v + h.as_str()
    } else {
        h + v.as_str()
    }) + "A"
}

fn num_coords(c: char) -> (usize, usize) {
    match c {
        '7' => (0, 0), '8' => (1, 0), '9' => (2, 0), 
        '4' => (0, 1), '5' => (1, 1), '6' => (2, 1), 
        '1' => (0, 2), '2' => (1, 2), '3' => (2, 2), 
        '0' => (1, 3), 'A' => (2, 3), _ => unreachable!()
    }
}

fn dir_coords(c: char) -> (usize, usize) {
    match c {
        '^' => (1, 0), 'A' => (2, 0),
        '<' => (0, 1), 'v' => (1, 1), '>' => (2, 1), 
        _ => unreachable!()
    }
}

#[cached]
fn deep_iter(s: String, steps: u64) -> usize {
    if steps == 0 {return s.len()}
    let bl = s.chars().any(|c| c.is_numeric());
    ("A".to_string() + &s)
        .chars()
        .map(|c| if bl {num_coords(c)} else {dir_coords(c)})
        .tuple_windows()
        .sum_by(|(c1, c2)| deep_iter(map_num((c1, c2), bl), steps - 1))
}

fn score2(s0: String, n_iter: u64) -> usize {
    s0[0..3].parse::<usize>().unwrap() * deep_iter(s0.clone(), n_iter)
}

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_21");

    // Your solution here...
    let sol1: u64 = lines.iter().sum_by(|s| score2(s.clone(), 3)) as u64;
    let sol2: u64 = lines.into_iter().sum_by(|s| score2(s, 26)) as u64;

    (Solution::U64(sol1), Solution::U64(sol2))
}
