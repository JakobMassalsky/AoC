use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

use crate::{etc::utils::{self, SumBy}, Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

fn get_dir(d: char) -> (isize, isize) {
    match d {
        '^' => (-1, 0),
        'v' => (1, 0),
        '>' => (0, 1),
        _ => (0, -1),
    }
}

fn move_robot<F>(mut warehouse: Vec<Vec<char>>, ins: &String, mut move_function: F) -> u64
    where
    F: FnMut(&mut Vec<Vec<char>>, (isize, isize), (isize, isize)) -> (isize, isize) {

    let mut x = warehouse.iter().position(|r| r.contains(&'@')).unwrap() as isize;
    let mut y = warehouse[x as usize].iter().position(|&r| r == '@').unwrap() as isize;

    for i in ins.chars() {
        let d = get_dir(i);
        (x, y) = match warehouse[(x + d.0) as usize][(y + d.1) as usize] {
            '#' => (x, y),
            'O' | '[' | ']' => move_function(&mut warehouse, (x, y), d),
            _ => (x+d.0, y+d.1)
        }
    }

    (0..warehouse.len()).cartesian_product(0..warehouse[0].len()).sum_by(|(x, y)| if ['O', '['].contains(&warehouse[x][y]) {x * 100 + y} else {0}) as u64
}

fn move_crate(warehouse: &mut Vec<Vec<char>>, (xx, yy): (isize, isize), (dx, dy): (isize, isize)) -> (isize, isize) {
    let (mut x, mut y) = (xx + dx, yy + dy);
    while warehouse[x as usize][y as usize] == 'O' {
        x += dx;
        y += dy;
    }
    match warehouse[x as usize][y as usize] {
        '#' => (xx, yy),
        _ => {
            warehouse[x as usize][y as usize] = 'O';
            warehouse[(xx + dx) as usize][(yy + dy) as usize] = '.';
            (xx + dx, yy + dy)
        }
    }
}

fn try_move_double_crate(warehouse: &mut Vec<Vec<char>>, (xx, yy): (isize, isize), (dx, dy): (isize, isize)) -> (isize, isize) {
    if dy != 0 {
        let mut y= yy + dy;
        let mut count = 0;
        while ['[', ']'].contains(&warehouse[xx as usize][y as usize]) {
            y += dy;
            count += 1;
        }
        if warehouse[xx as usize][y as usize] == '#' {
            return (xx, yy)
        } else if count > 0 {
            warehouse[xx as usize].remove(y as usize);
            warehouse[xx as usize].insert((yy + dy) as usize, '.');
        }
        return (xx, yy + dy)
    }
    
    let x= xx + dx;
    let mut moves = Vec::new();
    let mut look_q = VecDeque::new();
    look_q.push_back((xx, yy));
    while let Some(pos) = look_q.pop_front() {
        let nx = pos.0 + dx;
        if warehouse[nx as usize][pos.1 as usize] == ']' {
            look_q.push_back((nx, pos.1));
            look_q.push_back((nx, pos.1 - 1));
        } else if warehouse[nx as usize][pos.1 as usize] == '[' {
            look_q.push_back((nx, pos.1));
            look_q.push_back((nx, pos.1 + 1));
        } else if warehouse[nx as usize][pos.1 as usize] == '#' {
            return (xx, yy)
        }
        moves.push((pos, (nx, pos.1)));
    }
    for ((xo, yo), (xn, yn)) in moves.into_iter().unique().rev() {
        warehouse[xn as usize][yn as usize] = warehouse[xo as usize][yo as usize];
        warehouse[xo as usize][yo as usize] = '.';
    }
    (x, yy)
}

pub fn solve() -> SolutionPair {
    let lines = utils::read_lines("./input/input_15");
    let (grid, instructions) = lines.split(|s| s == "").next_tuple().unwrap();

    let warehouse = grid.into_iter().map(|s| s.chars().collect_vec()).collect_vec();
    let ins = instructions.concat();
    
    let warehouse2 = warehouse.iter()
        .map(|v| v.into_iter().map(|c| match c {
            'O' => ['[', ']'],
            _ => [*c, *c],
        }).flatten().collect())
        .collect();

    // Your solution here...
    let sol1: u64 = move_robot(warehouse, &ins, move_crate);
    let sol2: u64 = move_robot(warehouse2, &ins, try_move_double_crate);

    (Solution::U64(sol1), Solution::U64(sol2))
}
