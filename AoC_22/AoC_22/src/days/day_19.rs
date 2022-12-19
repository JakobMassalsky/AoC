use std::{collections::{HashSet, VecDeque}};

use itertools::Itertools;

use crate::{Solution, SolutionPair, etc::utils};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_19");
    let blueprints = lines.iter().map(|l| utils::extract_ints::<isize>(l, &[])).collect_vec();

    let m = bfs(vec![0, 0, 0, 0, 1, 0, 0, 0], &blueprints[1]);
    println!("{}", m);

    // Your solution here...
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn isKthBitSet(n: u32, k: u32) -> bool {
    (n >> k) & 1 > 0
}

fn try_buy(state: &Vec<isize>, orders: u32, blueprint: &Vec<isize>) -> Option<Vec<isize>> {
    let mut new_state = state.clone();
    if isKthBitSet(orders, 0) {
        new_state[0] -= blueprint[1];
        new_state[4] += 1;
    }
    if isKthBitSet(orders, 1) {
        new_state[0] -= blueprint[2];
        new_state[5] += 1;
    }
    if isKthBitSet(orders, 2) {
        new_state[0] -= blueprint[3];
        new_state[1] -= blueprint[4];
        new_state[6] += 1;
    }
    if isKthBitSet(orders, 3) {
        new_state[0] -= blueprint[5];
        new_state[2] -= blueprint[6];
        new_state[7] += 1;
    }
    if new_state[..3].iter().any(|&v| v < 0) {
        return None
    }
    new_state[0] += state[4];
    new_state[1] += state[5];
    new_state[2] += state[6];
    new_state[3] += state[7];
    return Some(new_state);
}

fn get_neighbours(state: &Vec<isize>, blueprint: &Vec<isize>) -> Vec<Vec<isize>> {
    let mut v = Vec::new();
    let mut new_state = state.clone();
    let mut remaining = state.clone();
    new_state[0] += state[4];
    new_state[1] += state[5];
    new_state[2] += state[6];
    new_state[3] += state[7];
    if state[0] < 4 {
        v.push(new_state.clone());
    }
    if state[2] >= blueprint[6] && state[0] >= blueprint[5] {
        new_state[0] -= blueprint[5];
        remaining[0] -= blueprint[5];
        new_state[2] -= blueprint[6];
        new_state[7] += 1;
        v.pop();
    } 
    if remaining[0] >= blueprint[3] && state[1] >= blueprint[4] {
        new_state[0] -= blueprint[3];
        remaining[0] -= blueprint[3];
        new_state[1] -= blueprint[4];
        new_state[6] += 1;
        v.pop();
    } 
    // v.push(new_state.clone());
    if remaining[0] >= blueprint[2] {
        let to_buy = remaining[0] / blueprint[2];
        new_state[0] -= to_buy * blueprint[2];
        remaining[0] -= to_buy * blueprint[2];
        new_state[5] += to_buy;
    } 
    if remaining[0] >= blueprint[1] {
        let to_buy = remaining[0] / blueprint[1];
        new_state[0] -= to_buy * blueprint[1];
        new_state[4] += to_buy;
    }
    if state != &new_state {
        v.push(new_state);
    }
    // for b in 0..16 {
    //     let nv = try_buy(&state, b, blueprint);
    //     if nv.is_some() {
    //         v.push(nv.unwrap());
    //     }
    // }
    v
}

fn bfs(start: Vec<isize>, blueprint: &Vec<isize>) -> u64 {
    let mut visit_queue = VecDeque::from_iter([(start.clone(), 0)]); 
    let mut visited = HashSet::new();
    visited.insert(start.clone()); 
    let mut max = 0;

    while !visit_queue.is_empty() {
        let (state, time) = visit_queue.pop_front().unwrap();
        max = max.max(state[3]);
        let new_time = time+1;
        if new_time >= 25 {
            continue;
        }
        for n in get_neighbours(&state, &blueprint) {
            for v in visited.clone().iter().filter(|v| (0..8).all(|i| v[i] <= n[i])) {
                visited.remove(v);
            }
            if visited.iter().any(|v| (0..8).all(|i| v[i] >= n[i])) {
                continue;
            }
            visited.insert(n.clone());
            visit_queue.push_back((n, new_time));
        }
    }
    max as u64
}
