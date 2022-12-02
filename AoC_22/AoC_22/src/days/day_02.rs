use crate::{Solution, SolutionPair, etc::utils};
use std::collections::{HashMap};
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let lines = utils::read_lines("./input/input_02");
    let wins = HashMap::from([('X', 'C'), ('Y', 'A'), ('Z', 'B')]);
    let points = HashMap::from([('X', 1), ('Y', 2), ('Z', 3)]);
    let points2 = HashMap::from([('X', 0), ('Y', 3), ('Z', 6)]);
    let mut  point_counter = 0;
    let mut  point_counter2 = 0;

    for line in &lines {
        let e = line.chars().nth(0).unwrap();
        let m = line.chars().last().unwrap();
        point_counter += points[&m];
        point_counter += if wins[&m] == e {
            6
        } else if m == e {
            3
        } else {0};

        point_counter2 += points2[&m];
        point_counter2 += if m == 'X' {
            match e {'A' => 3, 'B' => 1, _ => 2 }
        } else if m == 'Y' {
            match e {'A' => 1, 'B' => 2, _ => 3 }
        } else {
            match e {'A' => 2, 'B' => 3, _ => 1 }
        };
    }

    (Solution::U64(point_counter), Solution::U64(point_counter2))
}
