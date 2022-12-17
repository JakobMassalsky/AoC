use std::{collections::HashSet, vec};

use itertools::Itertools;

use crate::{Solution, SolutionPair, etc::utils};

///////////////////////////////////////////////////////////////////////////////

struct Rock {
    width: i32,
    height: i32,
    shape: Vec<[i32; 2]>
}

impl Rock {
    fn is_rested(&self, pivot: &[i32; 2], blocked: &HashSet<[i32; 2]>) -> bool {
        for [x, y] in &self.shape {
            if blocked.contains(&[x+pivot[0], y+pivot[1]-1]) {
                return true;
            }
        }
        false
    }
    fn is_vblocked(&self, pivot: &[i32; 2], blocked: &HashSet<[i32; 2]>, dir: i32) -> bool {
        if pivot[0]+dir < 0 || pivot[0]+dir+self.width > 7 {
            return true;
        }
        for [x, y] in &self.shape {
            if blocked.contains(&[x+pivot[0] + dir, y+pivot[1]]) {
                return true;
            }
        }
        false
    }
}

fn wind_dir(c: char) -> i32 {
    match c {
        '>' => 1,
        _ => -1
    }
}

pub fn solve() -> SolutionPair {

    let wind = include_str!("../../input/input_17").trim().chars().collect_vec();
    let wind_len = wind.len();

    let mut rocks = Vec::new();
    rocks.push(Rock {width: 4, height: 1, shape: vec![[0, 0], [1, 0], [2, 0], [3, 0]]});
    rocks.push(Rock {width: 3, height: 3, shape: vec![[1, 0], [0, 1], [1, 2], [2, 1]]}); // [1, 1], 
    rocks.push(Rock {width: 3, height: 3, shape: vec![[0, 0], [1, 0], [2, 0], [2, 1], [2, 2]]});
    rocks.push(Rock {width: 1, height: 4, shape: vec![[0, 0], [0, 1], [0, 2], [0, 3]]});
    rocks.push(Rock {width: 2, height: 2, shape: vec![[0, 0], [1, 0], [0, 1], [1, 1]]});

    let mut rock_counter = 0;
    let mut wind_counter = 0;
    let mut blocked = HashSet::new();
    let mut cur_max = 1;
    let mut max_stack = vec![];
    let mut sol1 = 0;

    for x in 0..7 {
        blocked.insert([x, 0]);
    }
    const LEN: usize = 20000;
    while rock_counter < LEN {
        let cur_rock = &rocks[rock_counter % 5];
        let mut x = 2;
        for _ in 0..4  {
            x = (x+wind_dir(wind[wind_counter % wind_len])).clamp(0, 7-cur_rock.width);
            wind_counter += 1;
        }
        let mut pivot = [x, cur_max];
        while !cur_rock.is_rested(&pivot, &blocked) {
            pivot[1] = pivot[1] - 1;
            let dir = wind_dir(wind[wind_counter % wind_len]);
            if !cur_rock.is_vblocked(&pivot, &blocked, dir) {
                pivot[0] += dir;
            }
            wind_counter += 1;
        }
        for p in &cur_rock.shape {
            blocked.insert([p[0] + pivot[0], p[1] + pivot[1]]);
        }
        cur_max = cur_max.max(pivot[1]+cur_rock.height);
        max_stack.push(cur_max);
        rock_counter += 1;
        if rock_counter == 2022 {
            sol1 = cur_max as u64 - 1;
        }
    }
    
    let diff = max_stack.windows(2).map(|h| h[1] - h[0]).collect_vec();
    let mut loop_size = 0;
    for i in 20..LEN {
        let n = diff.len()-i;
        if diff[n..] == diff[n-i..n] {
            loop_size = i;
            break;
        }
    };

    const DUR: usize = 1_000_000_000_000;
    let start = (DUR % loop_size) + loop_size * 10;
    let loop_height = (max_stack[start] - max_stack[start - loop_size]) as usize;

    let sol2: u64 = (((DUR-start) / loop_size) * loop_height + max_stack[start] as usize - 1) as u64;

    (Solution::U64(sol1), Solution::U64(sol2))
}
