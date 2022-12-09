use std::collections::HashSet;

use crate::{Solution, SolutionPair, etc::utils};
use rusttype::{self, Point, Vector};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_09");

    let mut head = Vector {x: 0, y: 0};
    let mut tail = head.clone();
    let mut visited = HashSet::new();
    visited.insert(tail);

    for (dir, dist) in lines.iter().map(|x| x.split_once(" ").unwrap()) {
        let dist: i32 = dist.parse().unwrap();
        let dir = match dir {
            "U" => Vector {x: 0, y: 1},
            "D" => Vector {x: 0, y: -1},
            "L" => Vector {x: -1, y: 0},
            _ => Vector {x: 1, y: 0},
        };
        head = head + Vector {x: dir.x * dist, y: dir.y * dist};
        let mut diff = head - tail;
        while diff.x.abs() > 1 || diff.y.abs() > 1 {
            tail = tail + Vector {x: diff.x.clamp(-1, 1), y: diff.y.clamp(-1, 1)};
            visited.insert(tail);
            diff = head - tail;
        }
    }

    // Your solution here...
    let sol1: u64 = visited.len() as u64;
    let sol2: u64 = 0;

    (Solution::U64(sol1), Solution::U64(sol2))
}
