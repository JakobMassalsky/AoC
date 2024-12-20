use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use crate::{etc::utils::{self, ParseAll}, Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

const D: i64 = 70;

fn get_neighbours((x, y, t): (i64, i64, i64)) -> [(i64, i64, i64); 4] {
    return [
        (x+1, y, t+1),
        (x-1, y, t+1),
        (x, y+1, t+1),
        (x, y-1, t+1),
    ];
}

fn part1(bytes: &HashMap<(i64, i64), i64>, age: i64) -> u64 {

    let mut visited = HashMap::new();
    let mut q = VecDeque::new();
    q.push_back((0, 0, 0));
    visited.insert((0, 0), 0);

    while let Some(node) = q.pop_front() {
        for (x, y, t) in get_neighbours(node) {
            if x < 0 || y < 0 || x > D || y > D ||
                visited.contains_key(&(x, y)) ||
                (bytes.get(&(x, y)).unwrap_or(&i64::MAX) <= &(age-1)) {continue;}
            if x == D && y == D {
                return t as u64;
            }
            visited.insert((x, y), t);
            q.push_back((x, y, t));
        }
    }

    0
}

pub fn solve() -> SolutionPair {

    let lines = HashMap::from_iter(utils::read_lines("./input/input_18")
        .into_iter()
        .map(|s| s.split(",").parse_all().next_tuple().unwrap())
        .enumerate()
        .map(|(i, (x, y))| ((x, y), i as i64)));

    let res = (2048..3000).position(|age| part1(&lines, age) == 0).unwrap();

    // Your solution here...
    let sol1: u64 = part1(&lines, 1024);
    let sol2: String = lines
        .into_iter()
        .filter(|(_, t)| *t == res as i64 + 2047)
        .map(|((x, y), _)| x.to_string() + "," + &y.to_string())
        .next()
        .unwrap();
    
    (Solution::U64(sol1), Solution::Str(sol2))
}
