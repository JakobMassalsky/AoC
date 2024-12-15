use std::io::{self, Write};

use itertools::Itertools;

use crate::{etc::utils::{self, extract_ints}, Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

const W: i64 = 101;
const H: i64 = 103;

fn move_robot(parms: &Vec<i64>, i: i64) -> (i64, i64) {
    ((parms[0] + parms[2] * i).rem_euclid(W), (parms[1] + parms[3] * i).rem_euclid(H))
}

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_14").into_iter()
        .map(|line| extract_ints(&line, &['-'])).collect_vec();

    let mut p = 0;

    for i in 0..10000 {
        let pos = lines.iter().map(|p| move_robot(p, i)).collect_vec();
        if pos.iter().map(|(x, y)| x * W + y).unique().sorted().tuple_windows()
            .any(|(i1, _, _, _, _, _, _, i8)| i1 + 7 == i8) {
            p = i;
            let mut img = vec![vec![' '; W as usize]; H as usize];
            for (x, y) in pos {
                img[y as usize][x as usize] = '#'
            }
            for row in img {
                println!("{}", row.into_iter().join(""));
            }
            break;
        }
    }

    let pos = lines.iter().map(|p| move_robot(p, 100)).collect_vec();

    let q1 = pos.iter().filter(|(x, y)| *x < W / 2 && *y < H / 2).count();
    let q2 = pos.iter().filter(|(x, y)| *x > W / 2 && *y < H / 2).count();
    let q3 = pos.iter().filter(|(x, y)| *x < W / 2 && *y > H / 2).count();
    let q4 = pos.iter().filter(|(x, y)| *x > W / 2 && *y > H / 2).count();

    // Your solution here...
    let sol1: u64 = (q1 * q2 * q3 * q4) as u64;
    let sol2: u64 = p as u64;

    (Solution::U64(sol1), Solution::U64(sol2))
}
