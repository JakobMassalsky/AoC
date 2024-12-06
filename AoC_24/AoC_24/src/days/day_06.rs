use std::collections::HashSet;

use itertools::Itertools;

use crate::{Solution, SolutionPair, etc::utils};

///////////////////////////////////////////////////////////////////////////////

fn next_dir((x, y): (i64, i64)) -> (i64, i64) {
    (y, -x)
}

fn do_calc(mut obs: HashSet<((i64, i64), (i64, i64))>, (mut x, mut y): (i64, i64), mut dir: (i64, i64), mut lines: Vec<Vec<char>>) -> bool {
    lines[(x+dir.0) as usize][(y+dir.1) as usize] = '#';
    dir = next_dir(dir);
    let (mut nextx, mut nexty) = (x+dir.0, y+dir.1);
    while nextx >= 0 && nexty >= 0 && (nextx as usize) < lines.len() && (nexty as usize) < lines.len() {
        if obs.contains(&((x, y), dir)) {
            return true;
        }
        obs.insert(((x, y), dir));
        if lines[nextx as usize][nexty as usize] == '#' {
            dir = next_dir(dir);
        } else {
            x = nextx;
            y = nexty;
        }
        (nextx, nexty) = (x+dir.0, y+dir.1);
    }

    false
}

fn part1(lines: Vec<Vec<char>>) -> u64 {

    let (xi, _) = lines.iter().find_position(|v| v.contains(&'^')).unwrap();
    let (yi, _) = lines[xi].iter().find_position(|v| v == &&'^').unwrap();
    let (mut x, mut y) = (xi as i64, yi as i64);

    let mut set: HashSet<(i64, i64)> = HashSet::new();
    let mut obs_set: HashSet<((i64, i64), (i64, i64))> = HashSet::new();

    let mut loops = 0;
    let mut dir: (i64, i64) = (-1, 0);
    let (mut nextx, mut nexty) = (x+dir.0, y+dir.1);
    while nextx >= 0 && nexty >= 0 && (nextx as usize) < lines.len() && (nexty as usize) < lines.len() {
        obs_set.insert(((x, y), dir));
        if lines[nextx as usize][nexty as usize] == '#' {
            dir = next_dir(dir);
        } else {
            set.insert((x, y));
            if !set.contains(&(x+dir.0, y+dir.1)) {
                if do_calc(obs_set.clone(), (x, y), dir, lines.clone()) {
                    loops += 1;
                }
            }
            x = nextx;
            y = nexty;
        }
        (nextx, nexty) = (x+dir.0, y+dir.1);
    }
    println!("{}", loops);

    set.len() as u64 + 1
}

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_06")
        .iter().map(|s| s.chars().collect()).collect();

    // Your solution here...
    let sol1: u64 = part1(lines);
    let sol2: u64 = 0;

    (Solution::U64(sol1), Solution::U64(sol2))
}
