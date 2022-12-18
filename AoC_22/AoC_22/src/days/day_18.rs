use std::collections::HashSet;

use itertools::Itertools;

use crate::{Solution, SolutionPair, etc::utils};

///////////////////////////////////////////////////////////////////////////////

fn get_free_faces(cube: &Vec<i64>, cubes: &HashSet<Vec<i64>>) -> u64 {
    let mut acc = 0;
    let mut c = cube.clone();
    c[0] += 1;
    acc += cubes.contains(&c) as u64;
    c[0] -= 2;
    acc += cubes.contains(&c) as u64;
    c = cube.clone();
    c[1] += 1;
    acc += cubes.contains(&c) as u64;
    c[1] -= 2;
    acc += cubes.contains(&c) as u64;
    c = cube.clone();
    c[2] += 1;
    acc += cubes.contains(&c) as u64;
    c[2] -= 2;
    acc += cubes.contains(&c) as u64;
    acc
}

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_18");

    let points = lines.iter().map(|l| utils::extract_ints::<i64>(l, &[])).collect_vec();
    let mut cubes = HashSet::new();
    points.iter().cloned().for_each(|p| {cubes.insert(p); });
    
    let count = points.iter().map(|p| get_free_faces(p, &cubes)).sum::<u64>();

    let sol1: u64 = points.len() as u64 * 6 - count;
    let sol2: u64 = 0;

    (Solution::U64(sol1), Solution::U64(sol2))
}
