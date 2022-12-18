use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

use crate::{Solution, SolutionPair, etc::utils};

///////////////////////////////////////////////////////////////////////////////

fn get_free_faces(cube: &Vec<i64>, cubes: &HashSet<Vec<i64>>) -> u64 {
    let mut acc = 0;
    let mut c = cube.clone();
    c[0] += 1;
    acc += !cubes.contains(&c) as u64;
    c[0] -= 2;
    acc += !cubes.contains(&c) as u64;
    c = cube.clone();
    c[1] += 1;
    acc += !cubes.contains(&c) as u64;
    c[1] -= 2;
    acc += !cubes.contains(&c) as u64;
    c = cube.clone();
    c[2] += 1;
    acc += !cubes.contains(&c) as u64;
    c[2] -= 2;
    acc += !cubes.contains(&c) as u64;
    acc
}

fn get_neighbours(seed: Vec<i64>, bounds: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut v = Vec::new();
    let mut c = seed.clone();
    c[0] = (seed[0]+1).clamp(bounds[0], bounds[3]);
    v.push(c.clone());
    c[0] = (seed[0]-1).clamp(bounds[0], bounds[3]);
    v.push(c.clone());
    c = seed.clone();
    c[1] = (seed[1]+1).clamp(bounds[1], bounds[4]);
    v.push(c.clone());
    c[1] = (seed[1]-1).clamp(bounds[1], bounds[4]);
    v.push(c.clone());
    c = seed.clone();
    c[2] = (seed[2]+1).clamp(bounds[2], bounds[5]);
    v.push(c.clone());
    c[2] = (seed[2]-1).clamp(bounds[2], bounds[5]);
    v.push(c.clone());
    v
}

fn fill(seed: Vec<i64>, bounds: &Vec<i64>, blocked: &HashSet<Vec<i64>>) -> HashSet::<Vec<i64>> {
    let mut visit_queue = VecDeque::new();
    let mut visited = HashSet::new();
    visit_queue.push_back(seed.clone());
    visited.insert(seed);

    while !visit_queue.is_empty() {
        let c = visit_queue.pop_front().unwrap();
        for n in get_neighbours(c.clone(), bounds) {
            if !blocked.contains(&n) && !visited.contains(&n) {
                visited.insert(n.clone());
                visit_queue.push_back(n);
            }
        }
    }
    visited
}

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_18");

    let points = lines.iter().map(|l| utils::extract_ints::<i64>(l, &[])).collect_vec();
    let mut cubes = HashSet::new();
    points.iter().cloned().for_each(|p| {cubes.insert(p); });
    
    let count = points.iter().map(|p| get_free_faces(p, &cubes)).sum::<u64>();

    let xmax = points.iter().map(|x| x[0]).max().unwrap()+1;
    let xmin = points.iter().map(|x| x[0]).min().unwrap()-1;
    let ymax = points.iter().map(|x| x[1]).max().unwrap()+1;
    let ymin = points.iter().map(|x| x[1]).min().unwrap()-1;
    let zmax = points.iter().map(|x| x[2]).max().unwrap()+1;
    let zmin = points.iter().map(|x| x[2]).min().unwrap()-1;

    let bounds = vec![xmin, ymin, zmin, xmax, ymax, zmax];
    let gas = fill(vec![xmin, ymin, zmin], &bounds, &cubes);
    let count2 = gas.iter().map(|p| get_free_faces(p, &gas)).sum::<u64>();

    let v = (xmax - xmin + 1) * (ymax - ymin + 1) * (zmax - zmin + 1);
    println!("Volume {}", v);
    println!("Gas {}", gas.len());
    println!("Lava {}", points.len());

    let sol1: u64 = count;
    // Subtract Outer Faces from Gas Cuboid
    let sol2: i64 = count2 as i64 - 
        (xmax - xmin + 1) * (ymax - ymin + 1) * 2 -
        (ymax - ymin + 1) * (zmax - zmin + 1) * 2 -
        (zmax - zmin + 1) * (xmax - xmin + 1) * 2;

    (Solution::U64(sol1), Solution::I64(sol2))
}
