use itertools::Itertools;

use crate::{etc::utils::{self, SumBy}, Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

fn step(map: &Vec<Vec<u32>>, old: u32, new: u32, c: (usize, usize)) -> Vec<(usize, usize)> {
    if new == old + 1 {
        if new == 9 {
            vec![c]
        } else {
            dfs(map, c)
        }
    } else {
        vec![]
    }
}

fn dfs(map: &Vec<Vec<u32>>, (cx, cy): (usize, usize)) -> Vec<(usize, usize)> {
    
    let mut set = vec![];
    let mut c = (cx.saturating_sub(1), cy);
    set.extend(step(map, map[cx][cy], map[c.0][c.1], c));
    c = (cx, cy.saturating_sub(1));
    set.extend(step(map, map[cx][cy], map[c.0][c.1], c));
    c = (cx, (cy+1).min(map[0].len()-1));
    set.extend(step(map, map[cx][cy], map[c.0][c.1], c));
    c = ((cx+1).min(map.len()-1), cy);
    set.extend(step(map, map[cx][cy], map[c.0][c.1], c));

    set
}

fn part1(map: &Vec<Vec<u32>>) -> u64 {
    (0..map.len()).cartesian_product(0..map[0].len()).sum_by(|(x, y)| {
        if map[x][y] == 0 {
            dfs(&map, (x, y)).into_iter().unique().count() as u64
        } else {0}
    })
}


fn part2(map: &Vec<Vec<u32>>) -> u64 {
    (0..map.len()).cartesian_product(0..map[0].len()).sum_by(|(x, y)| {
        if map[x][y] == 0 {
            dfs(&map, (x, y)).len() as u64
        } else {0}
    })
}

pub fn solve() -> SolutionPair {

    let lines: Vec<Vec<u32>> = utils::read_lines("./input/input_10").into_iter()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect_vec()).collect_vec();

    // Your solution here...
    let sol1: u64 = part1(&lines);
    let sol2: u64 = part2(&lines);

    (Solution::U64(sol1), Solution::U64(sol2))
}
