use std::{collections::HashSet, iter::repeat};

use crate::{Solution, SolutionPair, etc::utils};
use rusttype::{self, Point, Vector};

///////////////////////////////////////////////////////////////////////////////

fn move_rope(points: &mut [Vector<i32>]) {
    for i in 1..points.len() {
        let diff = points[i-1] - points[i];
        if diff.x.abs() > 1 || diff.y.abs() > 1 {
            points[i] = points[i] + Vector {x: diff.x.clamp(-1, 1), y: diff.y.clamp(-1, 1)};
        }
    }
}

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_09");

    let mut segments = [Vector {x: 0, y: 0}; 10];
    let mut visited = HashSet::new();
    let mut visited2 = HashSet::new();
    visited.insert(Vector {x: 0, y: 0});
    visited2.insert(Vector {x: 0, y: 0});

    for (dir, dist) in lines.iter().map(|x| x.split_once(" ").unwrap()) {
        let dir = match dir {
            "U" => Vector {x: 0, y: 1},
            "D" => Vector {x: 0, y: -1},
            "L" => Vector {x: -1, y: 0},
            _ => Vector {x: 1, y: 0},
        };

        for _ in 0..dist.parse().unwrap() {
            segments[0] = segments[0] + dir;
            move_rope(&mut segments);
            visited.insert(segments[1]);
            visited2.insert(segments[9]);
        }
    }

    let sol1: u64 = visited.len() as u64;
    let sol2: u64 = visited2.len() as u64;

    (Solution::U64(sol1), Solution::U64(sol2))
}
