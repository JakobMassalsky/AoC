use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::{etc::utils::{self, ParseAll, SumBy}, Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

fn r_next(mut input: usize) -> usize {
    input = (input ^ (input << 6)) & 16777215;
    input = input ^ (input >> 5);
    (input ^ (input << 11)) & 16777215
}

fn store_sequences(seqs: &mut Vec<usize>, mut secret: usize) -> usize {
    let mut last = secret % 10;
    let mut k = 0;
    let mut v = [false; 19_usize.pow(4)];

    for i in 0..2000 {
        secret = r_next(secret);
        let price = secret % 10;
        k = (k * 19 + price + 9 - last) % 130321;
        last = price;
        if i < 3 || v[k] { continue }
        v[k] = true;
        seqs[k] += price;
    }
    secret
}

pub fn solve() -> SolutionPair {
    let mut m = vec![0; 130321];

    // Your solution here...
    let sol1: u64 = utils::read_lines("./input/input_22")
                        .into_iter()
                        .parse_all()
                        .sum_by(|l| store_sequences(&mut m, l)) as u64;
    let sol2: u64 = m.into_iter().max().unwrap() as u64;

    (Solution::U64(sol1), Solution::U64(sol2))
}
