use itertools::Itertools;

use crate::{etc::utils::{self, SumBy}, Solution, SolutionPair};

use regex::Regex;

///////////////////////////////////////////////////////////////////////////////

fn part1(grid: &Vec<Vec<char>>) -> u64 {
    let w = grid.len();
    let h = grid[0].len();

    (0..w-3).cartesian_product(0..h-3).sum_by(|(x, y)|
        match (grid[x][y], grid[x+1][y+1], grid[x+2][y+2], grid[x+3][y+3]) {
            ('X', 'M', 'A', 'S') | ('S', 'A', 'M', 'X') => 1,
            _ => 0
        } + match (grid[x+3][y], grid[x+2][y+1], grid[x+1][y+2], grid[x][y+3]) {
            ('X', 'M', 'A', 'S') | ('S', 'A', 'M', 'X') => 1,
            _ => 0
        }
    ) + (0..w).cartesian_product(0..h-3).sum_by(|(x, y)|
        match (grid[x][y], grid[x][y+1], grid[x][y+2], grid[x][y+3]) {
            ('X', 'M', 'A', 'S') | ('S', 'A', 'M', 'X') => 1,
            _ => 0
        }
    ) + (0..w-3).cartesian_product(0..h).sum_by(|(x, y)|
        match (grid[x][y], grid[x+1][y], grid[x+2][y], grid[x+3][y]) {
            ('X', 'M', 'A', 'S') | ('S', 'A', 'M', 'X') => 1,
            _ => 0
        }
    )
}


fn part2(grid: Vec<Vec<char>>) -> u64 {
    (1..grid.len()-1).cartesian_product(1..grid.len()-1).sum_by(|(x, y)|
        match (grid[x][y], grid[x-1][y-1], grid[x+1][y+1], grid[x+1][y-1], grid[x-1][y+1]) {
            ('A', 'M', 'S','M','S') | ('A', 'M', 'S','S','M') | ('A', 'S', 'M','M','S') | ('A', 'S', 'M','S','M') => 1,
            _ => 0
        }
    )
}

pub fn solve() -> SolutionPair {

    let lines: Vec<Vec<char>> = utils::read_lines("./input/input_04").iter()
        .map(|line| line.chars().collect()).collect();

    // Your solution here...
    let sol1: u64 = part1(&lines);
    let sol2: u64 = part2(lines);

    (Solution::U64(sol1), Solution::U64(sol2))
}
