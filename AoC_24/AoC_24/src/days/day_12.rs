use std::{collections::{HashSet, VecDeque}, vec};

use itertools::Itertools;

use crate::{Solution, SolutionPair, etc::utils};

///////////////////////////////////////////////////////////////////////////////

fn get_step(map: &Vec<Vec<char>>, (x, y): (usize, usize), dir: (i64, i64), border: &mut Vec<((usize, usize), (i64, i64))>, queue: &mut VecDeque<(usize, usize)>) {
    let (nx, ny) = (x as i64 + dir.0, y as i64 + dir.1);
    if nx < 0 || nx > map.len() as i64 - 1 ||
        ny < 0 || ny > map.len() as i64 - 1 {
        border.push(((x, y), (nx, ny)));
    } else {
        if map[nx as usize][ny as usize] == map[x][y] {
            queue.push_back((nx as usize, ny as usize));
        } else {
            border.push(((x, y), (nx, ny)));
        }
    }
}

fn fill(map: &Vec<Vec<char>>, start: (usize, usize)) -> (u64, u64, HashSet<(usize, usize)>) {

    let mut visited = HashSet::new();
    let mut border: Vec<((usize, usize), (i64, i64))> = vec![];
    let mut queue = VecDeque::from([start]);

    while let Some(loc) = queue.pop_front() {
        if visited.contains(&loc) { continue; }
        visited.insert(loc);
        get_step(map, loc, (-1, 0), &mut border, &mut queue);
        get_step(map, loc, (1, 0), &mut border, &mut queue);
        get_step(map, loc, (0, -1), &mut border, &mut queue);
        get_step(map, loc, (0, 1), &mut border, &mut queue);
    }

    let l = map.len() as i64;

    let fences = border.iter()
        .map(|((x, y),(xo, yo))| {
            let (xi, yi) = (*x as i64, *y as i64);
            match (*xo - xi, *yo - yi) {
                (-1, 0) => (xi * l + yi) * -1 - 1,
                (1, 0) => (xi * l + yi) * 1 + 1,
                (0, -1) => (xi + yi * l) * -1 - 1000000,
                (0, 1) => (xi + yi * l) * 1 + 1000000,
                _ => unreachable!()
            }
        })
        .sorted()
        .tuple_windows()
        .filter(|(v1, v2)| *v1 != *v2 - 1)
        .count() as u64 + 1;

    (border.len() as u64 * visited.len() as u64, fences * visited.len() as u64, visited)
}


fn part(map: Vec<Vec<char>>) -> (u64, u64) {

    let mut visited = HashSet::new();
    let mut total1 = 0;
    let mut total2 = 0;

    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if visited.contains(&(x, y)) { continue; }
            let (score1, score2 , new_visits) = fill(&map, (x, y));
            visited.extend(new_visits);
            total1 += score1;
            total2 += score2;
        }
    }

    (total1, total2)
}

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_12")
        .into_iter()
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    // Your solution here...
    let (sol1, sol2) = part(lines);

    (Solution::U64(sol1), Solution::U64(sol2))
}
