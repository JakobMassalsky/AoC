use crate::{Solution, SolutionPair, etc::utils};
use std::collections::{HashSet, btree_set::Intersection};

///////////////////////////////////////////////////////////////////////////////

fn get_prio(c: u64) -> u64 {
    match c {
        65..=90 => c-38,
        _ => c-96
    }
}

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_03");
    let mut  s1 = 0;
    let mut  s2 = 0;
    for (b1, b2) in lines.clone().iter().map(|x| x.split_at(x.chars().count()/2)) {
        let set1: HashSet<char> = b1.chars().collect();
        let set2: HashSet<char> = b2.chars().collect();
        let mut i = set1.intersection(&set2);
        s1 += get_prio(*i.nth(0).unwrap() as u64);
    }
    
    for s in lines.chunks(3) {
        let sets = s.iter().map(|x| HashSet::from_iter(x.chars()));
        let i = sets.reduce(|acc: HashSet<char>, set: HashSet<char> | HashSet::from_iter(acc.intersection(&set).copied()));
        s2 += get_prio(*i.unwrap().iter().next().unwrap() as u64);
    }
    // Your solution here...
    
    (Solution::U64(s1), Solution::U64(s2))
}
