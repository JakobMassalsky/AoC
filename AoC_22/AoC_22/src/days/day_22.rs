use ndarray::Array2;
use rusttype::Vector;

use crate::{Solution, SolutionPair, etc::utils};

///////////////////////////////////////////////////////////////////////////////

fn get_char_val(c: char) -> i8 {
    match c {
        '.' => -1,
        '#' => 1,
        ' ' => 0,
        _ => unreachable!()
    }
}

fn get_dir_vec(mut d: i32) -> Vector<i32> {
    d = d % 4;
    if d < 0 { d += 4; }
    match d {
        0 => Vector {x: 1, y: 0},
        1 => Vector {x: 0, y: 1},
        2 => Vector {x: -1, y: 0},
        3 => Vector {x: 0, y: -1},
        _ => unreachable!()
    }
}

fn find_opposite(dir: Vector<i32>, orig_pos: Vector<i32>, map: &Array2<i8>) -> Vector<i32> {
    let w = map.raw_dim()[0] as i32;
    let h = map.raw_dim()[1] as i32;
    let mut pos = orig_pos;
    loop {
        let new_pos = pos-dir;
        if new_pos.x < 0 || new_pos.y < 0 || new_pos.x >= w || new_pos.y >= h || map[[new_pos.x as usize, new_pos.y as usize]] == 0 {
            if map[[pos.x as usize, pos.y as usize]] == -1 {
                return pos;
            } else if map[[pos.x as usize, pos.y as usize]] == 1 {
                return orig_pos;
            }
        }
        pos = new_pos;
    }
}

fn move_dir(dir: Vector<i32>, dist: i32, mut pos: Vector<i32>, map: &Array2<i8>) -> Vector<i32> {
    let w = map.raw_dim()[0] as i32;
    let h = map.raw_dim()[1] as i32;
    for _ in 0..dist {
        let new_pos = pos+dir;
        if new_pos.x < 0 || new_pos.y < 0 || new_pos.x >= w || new_pos.y >= h {
            pos = find_opposite(dir, pos, map);
        } else {
            let val = map[[new_pos.x as usize, new_pos.y as usize]];
            if val == 1 {
                return pos;
            }
            if val == 0 {
                pos = find_opposite(dir, pos, map);
            } else {
                pos = new_pos;
            }
        }
    }
    pos
}

fn move_dir_2(dir: Vector<i32>, dist: i32, mut pos: Vector<i32>, map: &Array2<i8>) -> Vector<i32> {
    const a: i32 = 4;
    let w = map.raw_dim()[0] as i32;
    let h = map.raw_dim()[1] as i32;
    let face = (pos.x / a, pos.y / a);
    for _ in 0..dist {
        let new_pos = pos+dir;
        if new_pos.x < 0 || new_pos.y < 0 || new_pos.x >= w || new_pos.y >= h {
            pos = find_opposite(dir, pos, map);
        } else {
            let val = map[[new_pos.x as usize, new_pos.y as usize]];
            if val == 1 {
                return pos;
            }
            if val == 0 {
                pos = find_opposite(dir, pos, map);
            } else {
                pos = new_pos;
            }
        }
    }
    pos
}

// fn invert(pos: Vector<i32>, a: i32) -> Vector<i32> {
//     Vector { x: a - (pos.x % a) - 1, y: () }
// }

fn move_face(old: (i32, i32), new: (i32, i32), pos: Vector<i32>, a: i32) -> Vector<i32> {
    match (old, new) {
        ((0, 1), (0, 0)) => Vector {x: 3 * a - pos.x - 1, y: 0},
        ((2, 0), (2, -1)) => Vector {x: a - (pos.x % a) - 1, y: a},
        ((0, 1), (-1, 1)) => Vector {x: 4 * a - (pos.y % a) - 1, y: 4 * a - 1},
        ((3, 2), (3, 3)) => Vector {x: 0, y: 4 * a - 1},
        _ => pos
    }
}

fn part_1(map: &Array2<i8>, mut pos: Vector<i32>, dirs: &String) -> i32 {
    let mut int_acc = "".to_string();
    let mut dir = 0;

    for (i, c) in dirs.char_indices() {
        if c.is_numeric() {
            int_acc.push(c);
        } else {
            let dist = int_acc.parse().unwrap();
            pos = move_dir(get_dir_vec(dir), dist, pos, &map);
            dir += match c {
                'R' => 1,
                _ => -1
            };
            int_acc.clear();
            if map[[pos.x as usize, pos.y as usize]] != -1 {
                println!("Failed: {:?}", pos);
            }
        }
    }
    let dist = int_acc.parse().unwrap();
    pos = move_dir(get_dir_vec(dir), dist, pos, &map);
    
    dir = dir % 4;
    if dir < 0 { dir += 4; }
    1000 * (pos.y + 1) + 4 * (pos.x + 1) + dir
}

fn part_2(map: &Array2<i8>, mut pos: Vector<i32>, dirs: &String) -> i32 {
    let mut int_acc = "".to_string();
    let mut dir = 0;
    
    for (i, c) in dirs.char_indices() {
        if c.is_numeric() {
            int_acc.push(c);
        } else {
            let dist = int_acc.parse().unwrap();
            pos = move_dir_2(get_dir_vec(dir), dist, pos, &map);
            dir += match c {
                'R' => 1,
                _ => -1
            };
            int_acc.clear();
            if map[[pos.x as usize, pos.y as usize]] != -1 {
                println!("Failed: {:?}", pos);
            }
            if i < 9 {
                println!("{:?}, dir: {:?} {}", pos, get_dir_vec(dir), map[[pos.x as usize, pos.y as usize]]);
            }
        }
    }
    let dist = int_acc.parse().unwrap();
    pos = move_dir_2(get_dir_vec(dir), dist, pos, &map);
    
    dir = dir % 4;
    if dir < 0 { dir += 4; }
    1000 * (pos.y + 1) + 4 * (pos.x + 1) + dir
}

pub fn solve() -> SolutionPair {

    let mut lines = utils::read_lines("./input/input_22");

    let dirs = lines.pop().unwrap();
    
    let width = lines.iter().map(|l| l.len()).max().unwrap();
    let mut map = Array2::zeros([width, lines.len()]);

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.char_indices() {
            map[[x, y]] = get_char_val(c);
        }
    }

    // println!("{:?}", map);
    let s = part_1(&map, Vector {x: lines[0].chars().position(|c| c != ' ').unwrap() as i32, y: 0}, &dirs);

    // Your solution here...
    let sol1: u64 = s as u64;
    let sol2: u64 = 0;
    
    (Solution::U64(sol1), Solution::U64(sol2))
}
