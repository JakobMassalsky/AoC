use std::{collections::HashMap, u64};

use itertools::Itertools;
use priority_queue::PriorityQueue;

use crate::{etc::utils::{self, SumBy}, Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

fn get_neighbours((x, y): (usize, usize)) -> [(usize, usize); 4] {
    [(x-1, y), (x+1, y), (x, y+1), (x, y-1)]
}

fn dijkstra(map: &Vec<Vec<char>>, start: (usize, usize)) -> HashMap<(usize, usize), u64> {
    
    let mut q = PriorityQueue::new();
    q.push(start, u64::MAX);
    let mut visited: HashMap<(usize, usize), u64> = HashMap::new();
    visited.insert(start, u64::MAX);

    while let Some((node, prio)) = q.pop() {
        for (x, y) in get_neighbours(node) {
            let nc = prio - 1;
            match map[x][y] {
                '#' => (),
                _ => {
                    if map[x][y] == '#' { continue }
                    if let Some(&vc) = visited.get(&(x, y)) {
                        if vc >= nc { continue }
                    }
                    q.push((x, y), nc);
                    visited.insert((x, y), nc);
                }
            }
        }
    }
    
    visited
}

fn test_cheats(distance_to_end: &HashMap<(usize, usize), u64>, distance_from_start: &HashMap<(usize, usize), u64>, maxd: u64, min_cheat: u64, cheat_duration: usize) -> u64 {
    let mut saves = HashMap::new();
    for ((x, y), d) in distance_from_start {
        for ((nx, ny), nd) in distance_to_end.iter().filter(|((x_e, y_e), d_e)| 
            x_e.abs_diff(*x) + y_e.abs_diff(*y) <= cheat_duration && 
            (u64::MAX - d) + x_e.abs_diff(*x) as u64 + y_e.abs_diff(*y) as u64 + (u64::MAX - *d_e) <= maxd - min_cheat
        ) {
            let cheat_d = maxd - ((u64::MAX - d) + nx.abs_diff(*x) as u64 + ny.abs_diff(*y) as u64 + (u64::MAX - *nd));
            if let Some(v) = saves.get_mut(&cheat_d) {
                *v += 1;
            } else {
                saves.insert(cheat_d, 1);
            }
        }
    }
    saves.into_iter().sum_by(|(_, v)| v) as u64
}

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_20")
        .into_iter().map(|s| s.chars().collect_vec())
        .collect_vec();

    let s = (0..lines.len()).cartesian_product(0..lines[0].len()).find(|(x, y)| lines[*x][*y] == 'S').unwrap();
    let e = (0..lines.len()).cartesian_product(0..lines[0].len()).find(|(x, y)| lines[*x][*y] == 'E').unwrap();
    
    let d_s = dijkstra(&lines, s);
    let d_e = dijkstra(&lines, e);
    
    // Your solution here...
    let sol1: u64 = test_cheats(&d_e, &d_s, u64::MAX-d_e[&s], 100, 2);
    let sol2: u64 = test_cheats(&d_e, &d_s, u64::MAX-d_e[&s], 100, 20);

    (Solution::U64(sol1), Solution::U64(sol2))
}
