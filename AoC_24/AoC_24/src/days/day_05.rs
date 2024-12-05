use std::collections::{HashMap, HashSet};

use crate::{etc::utils::{self, ParseAll}, Solution, SolutionPair};

use itertools::{enumerate, Itertools};

///////////////////////////////////////////////////////////////////////////////

fn part1(lines: Vec<String>) -> (u64, u64) {

    let (rules, updates) = lines.split(|s| s == "").next_tuple().unwrap();

    let mut map: HashMap<u64, HashSet<u64>> = (0..100).into_iter().map(|i| (i, HashSet::new())).collect();
    for rule in rules {
        let (cond, i): (u64, u64) = rule.split(" | ").parse_all().next_tuple().unwrap();
        map.get_mut(&i).unwrap().insert(cond);
    }

    let mut s1 = 0;
    let mut s2 = 0;

    for mut update in updates.iter().map(|s| s.split(", ").parse_all().collect::<Vec<u64>>()) {
        let mut valid = true;
        let mut i = 0;
        while i < update.len()-1 {
            let mi = map.get(&update[i]).unwrap();
            if let Some((index, _)) = update.iter().enumerate().skip(i).rev().find(|(_, &n)| mi.contains(&n)) {
                let v = update.remove(i);
                update.insert(index, v);
                valid = false;
            } else {
                i += 1
            }
        }

        if valid {
            s1 += update[(update.len()-1)/2];
        } else {
            s2 += update[(update.len()-1)/2];
        }
    }

    (s1, s2)
}

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_05");

    // Your solution here...
    let (sol1, sol2) = part1(lines);

    (Solution::U64(sol1), Solution::U64(sol2))
}
