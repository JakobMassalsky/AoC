use std::collections::{HashSet, VecDeque};
use itertools::Itertools;

use crate::{Solution, SolutionPair, etc::utils};

///////////////////////////////////////////////////////////////////////////////

fn get_neighbours(p: [usize; 2], map: &Vec<Vec<char>>, visited: &HashSet<[usize; 2]>) -> Vec<[usize; 2]> {
    let [x, y] = p;
    let a = match map[x][y] {
        'S' => 'a',
        _ => map[x][y]} as u8;
    let mut v = Vec::new();
    for (xn, yn) in [(x.saturating_sub(1), y), (x, y.saturating_sub(1)), ((x+1).min(map.len()-1), y), (x, (y+1).min(map[0].len()-1))] {
        if xn == x && yn == y {continue}
        if visited.contains(&[xn, yn]) {continue}
        if if map[xn][yn] == 'E' {'z'} else {map[xn][yn]} as u8 > a+1 {continue}
        v.push([xn, yn]);
    }
    v
}

fn bfs(start: [usize; 2], map: &Vec<Vec<char>>, goal: char) -> u64 {
    let mut visit_queue = VecDeque::from_iter([(start, 0)]);
    let mut visited = HashSet::from_iter([start]);

    while !visit_queue.is_empty() {
        let (p, prio) = visit_queue.pop_front().unwrap();
        if map[p[0]][p[1]] == goal {return prio;}
        for neighbour in get_neighbours(p, &map, &visited) {
            visited.insert(neighbour);
            visit_queue.push_back((neighbour, prio+1));
        }
    }
    u64::MAX
}

pub fn solve() -> SolutionPair {

    let map = utils::read_lines("./input/input_12")
        .iter().map(|l| l.chars().collect_vec())
        .collect_vec();
        
    let mut sol1: u64 = 0;
    let mut lengths = Vec::new();

    for (x, v) in map.iter().enumerate() {
        for (y, &c) in v.iter().enumerate() {
            if c == 'a' {
                lengths.push(bfs([x, y], &map, 'E'));
            }
            if c == 'S' {
                sol1 = bfs([x, y], &map, 'E');
            }
        }
    }

    let sol2: u64 = *lengths.iter().min().unwrap();

    (Solution::U64(sol1), Solution::U64(sol2))
}
