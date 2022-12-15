use std::{collections::HashSet, num};

use itertools::Itertools;

use crate::{Solution, SolutionPair, etc::utils};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_14");
    let paths = lines.iter().map(|s| utils::extract_ints::<i16>(s, &[])).collect_vec();

    let mut points = HashSet::new();

    for path in paths {
        let mut last_point = [path[0], path[1]];
        for p in path.chunks(2).skip(1) {
            for px in last_point[0].min(p[0])..=last_point[0].max(p[0]) {
                for py in last_point[1].min(p[1])..=last_point[1].max(p[1]) {
                    points.insert([px, py]);
                }
            }
            last_point = [p[0], p[1]];
        }
    }
    let y_border = *points.iter().map(|[_, y]| y).max().unwrap();
    let num_walls = points.len();

    let start = [500, 0];
    let mut curr = start;
    let mut sol1 = 0;
    while curr[1] < y_border {
        if !points.contains(&[curr[0], curr[1] + 1]) {
            curr = [curr[0], curr[1] + 1];
        } else if !points.contains(&[curr[0] - 1, curr[1] + 1]) {
            curr = [curr[0] - 1, curr[1]+1];
        } else if !points.contains(&[curr[0] + 1, curr[1] + 1]) {
            curr = [curr[0] + 1, curr[1] + 1];
        } else {
            points.insert(curr);
            curr = start;
            sol1 += 1;
        }
    }

    println!("{}", points.len());

    curr = start;
    while !points.contains(&start) {
        let ny = curr[1] + 1;
        if ny == y_border + 2 {
            points.insert(curr);
            curr = start;
            continue;
        }
        if !points.contains(&[curr[0], ny]) {
            curr = [curr[0], ny];
        } else if !points.contains(&[curr[0] - 1, ny]) {
            curr = [curr[0] - 1, ny];
        } else if !points.contains(&[curr[0] + 1, ny]) {
            curr = [curr[0] + 1, ny];
        } else {
            points.insert(curr);
            curr = start;
        }
    }

    let sol2: u64 = (points.len() - num_walls) as u64;

    (Solution::U64(sol1), Solution::U64(sol2))
}
