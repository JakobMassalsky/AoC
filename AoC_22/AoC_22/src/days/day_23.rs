use std::collections::{HashMap, VecDeque, HashSet};

use ndarray::{Array2, Axis, Array, s};

use crate::{Solution, SolutionPair, etc::utils};

///////////////////////////////////////////////////////////////////////////////

fn check_move(x: isize, y: isize, dirs: VecDeque<u8>, elfes: &HashSet<[isize; 2]>) -> Option<[isize; 2]> {
    let mut i = dirs.iter().map(|dir| match dir {
        0 => if !elfes.contains(&[x-1, y-1]) && !elfes.contains(&[x, y-1]) && !elfes.contains(&[x+1, y-1]) {
            Some([x, y-1])
        } else { None }
        1 => if !elfes.contains(&[x-1, y+1]) && !elfes.contains(&[x, y+1]) && !elfes.contains(&[x+1, y+1]) {
            Some([x, y+1])
        } else { None }
        2 => if !elfes.contains(&[x-1, y-1]) && !elfes.contains(&[x-1, y]) && !elfes.contains(&[x-1, y+1]) {
            Some([x-1, y])
        } else { None }
        3 => if !elfes.contains(&[x+1, y-1]) && !elfes.contains(&[x+1, y]) && !elfes.contains(&[x+1, y+1]) {
            Some([x+1, y])
        } else { None }
        _ => unreachable!()
    });
    if i.clone().all(|v| v.is_some()) {return None}
    let v = i.find(|v| v.is_some());
    if v.is_some() { return v.unwrap() }
    return None
}

fn min_max(elfes: &HashSet<[isize; 2]>) -> isize {
    let xmin = elfes.iter().map(|[x, _]| x).min().unwrap();
    let xmax = elfes.iter().map(|[x, _]| x).max().unwrap();
    let ymin = elfes.iter().map(|[_, y]| y).min().unwrap();
    let ymax = elfes.iter().map(|[_, y]| y).max().unwrap();
    (ymax - ymin + 1) * (xmax - xmin + 1)
}

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_23");

    let mut elfes = HashSet::new();
    let mut move_suggestions: HashMap<[isize; 2], usize> = HashMap::new();
    let mut to_move = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.char_indices() {
            if c == '#' {
                elfes.insert([x as isize, y as isize]);
            }
        }
    }
    
    let mut dirs = VecDeque::from_iter([0, 1, 2, 3]);
    let mut round = 1;
    let mut last = elfes.clone();
    
    let mut sol1: u64 = 0;
    loop {
        to_move.clear();
        for &[x, y] in elfes.iter() {
            let n = check_move(x, y, dirs.clone(), &elfes);
            if n.is_some() {
                let i = move_suggestions.get_mut(&n.unwrap());
                if i.is_some() {
                    let v = i.unwrap();
                    *v = *v+1;
                } else { move_suggestions.insert(n.unwrap(), 1); };
                to_move.insert([x, y], n.unwrap());
            }
        }
        // println!("{:?}", to_move);
        for (k, v) in to_move.iter() {
            if move_suggestions[v] > 1 { continue; }
            elfes.remove(k);
            elfes.insert(*v);
        }
        move_suggestions.clear();
        let v = dirs.pop_front().unwrap();
        dirs.push_back(v);
        if round == 10 {
            sol1 = (min_max(&elfes) - elfes.len() as isize) as u64;
        }
        if elfes == last {
            break;
        } else {
            last = elfes.clone();
            round += 1;
        }
        if round % 100 == 0 {
            println!("{round}");
        }
    }


    // Your solution here...
    let sol2: u64 = round;

    (Solution::U64(sol1), Solution::U64(sol2))
}
