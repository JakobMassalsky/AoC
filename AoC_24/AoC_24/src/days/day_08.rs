use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::{Solution, SolutionPair, etc::utils};

///////////////////////////////////////////////////////////////////////////////

fn part1(set: &HashSet<(i64, i64)>, w: usize, h: usize) -> HashSet<(i64, i64)> {

    HashSet::from_iter((set.iter()).cartesian_product(set.iter()).map(|((x1, y1), (x2, y2))| {
        if x1 == x2 && y1 == y2 {[(-1, -1), (-1, -1)]} else {
        let dx = *x1 - *x2;
        let dy = *y1 - *y2;
        [(*x1 + dx, *y1 + dy), (*x2 - dx, *y2 - dy)]}
    }).flatten().filter(|(x, y)| *x >= 0 && *y >= 0 && *x < w as i64 && *y < h as i64))
}

fn part2(set: &HashSet<(i64, i64)>, w: usize, h: usize) -> HashSet<(i64, i64)> {
    (set.iter()).cartesian_product(set.iter()).fold(HashSet::new(), |mut s: HashSet<(i64, i64)>, ((x1, y1), (x2, y2))| {
        if x1 == x2 && y1 == y2 {s} else {
        let dx = *x1 - *x2;
        let dy = *y1 - *y2;
        let mut xn1 = *x1;
        let mut yn1 = *y1;
        while xn1 >= 0 && xn1 < w as i64 && yn1 >= 0 && yn1 < h as i64 {
            s.insert((xn1, yn1));
            xn1 += dx;
            yn1 += dy;
        }
        xn1 = *x2;
        yn1 = *y2;
        while xn1 >= 0 && xn1 < w as i64 && yn1 >= 0 && yn1 < h as i64 {
            s.insert((xn1, yn1));
            xn1 -= dx;
            yn1 -= dy;
        }
        s
        }
    })
}

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_08").iter().map(|l| l.chars().collect_vec()).collect_vec();

    let mut m: HashMap<char, HashSet<(i64, i64)>> = HashMap::new();

    let w = lines.len();
    let h = lines[0].len();
    for (x, y) in (0..w).cartesian_product(0..h) {
        let c = lines[x][y];
        if c != '.' {
            if let Some(s) = m.get_mut(&c) {
                s.insert((x as i64, y as i64));
            } else {
                m.insert(c,  [(x as i64, y as i64)].into());
            }
        }
    }

    let antinodes1 = m.values()
        .fold(HashSet::new(), |mut acc: HashSet<(i64, i64)>, set| {
            acc.extend(part1(set, w, h).into_iter()); 
            acc
        });

    let antinodes2 = m.values()
        .fold(HashSet::new(), |mut acc: HashSet<(i64, i64)>, set| {
            acc.extend(part2(set, w, h).into_iter()); 
            acc
        });

    // Your solution here...
    let sol1: u64 = antinodes1.len() as u64;
    let sol2: u64 = antinodes2.len() as u64;

    (Solution::U64(sol1), Solution::U64(sol2))
}
