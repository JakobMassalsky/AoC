use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::{etc::utils::{self, ParseAll}, Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

fn mix_prune(input: u64, n2: u64) -> u64 {
    (input ^ n2) & ((1 << 24) - 1)
}

fn r_next(input: u64) -> u64 {
    let n1 = mix_prune(input, input << 6);
    let n2 = mix_prune(n1, n1 >> 5);
    mix_prune(n2, n2 << 11)
}

fn store_sequences(seqs: &mut HashMap<(i8, i8, i8, i8), u64>, mut secret: u64) {
    let mut visited = HashSet::new();
    let mut q = [0, 0, 0, 0];
    let mut last = secret;

    for i in 0..2000 {
        secret = r_next(secret);
        q[i%4] = (secret % 10) as i8 - (last % 10) as i8;
        last = secret;

        if i >= 3 {
            let k = (q[(i-3)%4], q[(i-2)%4], q[(i-1)%4], q[i%4]);
            if visited.contains(&k) { continue; }
            visited.insert(k);
            if let Some(n) = seqs.get_mut(&k) {
                *n += secret % 10;
            } else {
                seqs.insert(k, secret % 10);
            }
        }
    }
}

pub fn solve() -> SolutionPair {

    let lines: Vec<u64> = utils::read_lines("./input/input_22").into_iter().parse_all().collect();

    let l2 = lines.clone().into_iter().map(|mut n| {
        for _ in 0..2000 { n = r_next(n); }
        n
    }).collect_vec();

    let mut m = HashMap::new();
    for l in lines {
        store_sequences(&mut m, l);
    }

    // Your solution here...
    let sol1: u64 = l2.into_iter().sum();
    let sol2: u64 = *m.values().max().unwrap() as u64;
    // 2123 -> low

    (Solution::U64(sol1), Solution::U64(sol2))
}
