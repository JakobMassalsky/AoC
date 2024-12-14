use itertools::Itertools;

use crate::{etc::utils::{self, extract_ints, SumBy}, Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

fn get_coins((a, b, goal): (Vec<i64>, Vec<i64>, Vec<i64>)) -> u64 {

    let mut price = 0;
    let (mut curx, mut cury) = (goal[0], goal[1]);

    while (curx % b[0] != 0 || cury % b[1] != 0 || (curx / b[0] != cury / b[1])) && price < 300 && curx >= a[0] && cury >= a[1] {
        price += 3;
        curx -= a[0];
        cury -= a[1];
    }
    if curx % b[0] != 0 || cury % b[1] != 0 || curx / b[0] > 100 {
        0
    } else {
        (price + curx / b[0]) as u64
    }

}

fn get_coins2((a, b, goal): (Vec<i64>, Vec<i64>, Vec<i64>)) -> u64 {

    let (ax, ay) = (a[0], a[1]);
    let (bx, by) = (b[0], b[1]);
    let (x, y) = (goal[0] + 10000000000000, goal[1] + 10000000000000);
    
    let m = (ay*x - ax*y)/(ay*bx - ax*by);
    let n = (x - bx * m) / ax;
    
    if ax * n + bx * m == x && ay * n + by * m == y {
        (m + 3 * n) as u64
    } else {
        0
    }
}

pub fn solve() -> SolutionPair {

    let lines: Vec<Vec<i64>> = utils::read_lines("./input/input_13").into_iter()
        .map(|line| extract_ints(&line, &[]))
        .enumerate()
        .filter_map(|(i, e)| if i % 4 != 3 { Some(e) } else { None })
        .collect_vec();

    // println!("{:?}", lines);

    // Your solution here...
    let sol1: u64 = lines.clone().into_iter().tuples().sum_by(get_coins);
    let sol2: u64 = lines.into_iter().tuples().sum_by(get_coins2);

    (Solution::U64(sol1), Solution::U64(sol2))
}
