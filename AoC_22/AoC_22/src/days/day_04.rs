use crate::{Solution, SolutionPair, etc::utils};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let lines = utils::read_lines("./input/input_04");
    
    let mut sol1: u64 = 0;
    let mut sol2: u64 = 0;
    for line in lines.iter().map(|x| x.split(&[',', '-'][..]).collect::<Vec<&str>>()) {
        let a1: u64 = line[0].parse().unwrap();
        let a2: u64 = line[1].parse().unwrap();
        let b1: u64 = line[2].parse().unwrap();
        let b2: u64 = line[3].parse().unwrap();

        if (a1 <= b1 && a2 >= b2) || (b1 <= a1 && b2 >= a2) {
            sol1 += 1;
        }

        if !((a2 < b1) || (a1 > b2)) {
            sol2 += 1;
        }
    }

    (Solution::U64(sol1), Solution::U64(sol2))
}
