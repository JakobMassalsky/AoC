use crate::{Solution, SolutionPair, etc::utils};


pub fn solve() -> SolutionPair {    
    let lines = utils::read_lines("./input/input_01");
    let mut sums: Vec<i32> = Vec::new();
    let mut sum = 0;
    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        if line.is_empty() {
            sums.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }
    sums.sort_by(|a, b| b.cmp(a));

    let sol1: i32 = sums[sums.len()-1];
    let sol2: i32 = sums[0..3].iter().sum();

    (Solution::I32(sol1), Solution::I32(sol2))
}
