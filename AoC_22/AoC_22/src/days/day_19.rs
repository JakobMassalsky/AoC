use std::{collections::{HashSet, VecDeque}};

use itertools::Itertools;

use crate::{Solution, SolutionPair, etc::utils};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_19");
    let blueprints = lines.iter().map(|l| utils::extract_ints::<isize>(l, &[])).collect_vec();

    let m = bfs(vec![0, 0, 0, 0, 1, 0, 0, 0], &blueprints[0]);
    println!("{}", m);

    // Your solution here...
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn theo_max(state: &Vec<isize>, t: isize) -> isize {
    return state[3] + state[7] * (24 - t);
}

fn is_kth_bit_set(n: u32, k: u32) -> bool {
    (n >> k) & 1 > 0
}

fn try_buy(state: &Vec<isize>, orders: u32, blueprint: &Vec<isize>, new_g: isize, new_o: isize) -> Option<Vec<isize>> {
    let mut new_state = state.clone();
    if new_state[0] > 4 && orders == 0 {
        return None;
    }
    if is_kth_bit_set(orders, 0) {
        new_state[0] -= blueprint[1];
        new_state[4] += 1;
    }
    if is_kth_bit_set(orders, 1) {
        new_state[0] -= blueprint[2];
        new_state[5] += 1;
    }
    if new_state[0] < 0 {
        return None
    }
    new_state[0] += state[4];
    new_state[1] += state[5];
    new_state[2] += state[6]-new_o;
    new_state[3] += state[7]-new_g;
    return Some(new_state);
}

fn get_neighbours(state: &Vec<isize>, blueprint: &Vec<isize>, min_ore: isize, max_ore:isize) -> Vec<Vec<isize>> {
    let mut v = Vec::new();
    let mut new_state = state.clone();
    // nothing can be built
    if state[0] < min_ore {
        new_state[0] += state[4];
        new_state[1] += state[5];
        new_state[2] += state[6];
        new_state[3] += state[7];
        v.push(new_state);
        return v;
    }
    if state[2] >= blueprint[6] && state[0] >= blueprint[5] {
        new_state[0] -= blueprint[5];
        new_state[2] -= blueprint[6];
        new_state[7] += 1;
    } 
    if new_state[0] >= blueprint[3] && state[1] >= blueprint[4] {
        new_state[0] -= blueprint[3];
        new_state[1] -= blueprint[4];
        new_state[6] += 1;
    }
    for b in 0..4 {
        let nv = try_buy(&new_state, b, blueprint, new_state[7]-state[7], new_state[6]-state[6]);
        if nv.is_some() {
            v.push(nv.unwrap());
        }
    }
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
        if new_time >= 20 {
            continue;
        }
        for n in get_neighbours(&state, &blueprint, 2, 4) {
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
