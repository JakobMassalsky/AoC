use crate::{Solution, SolutionPair, etc::utils};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {

    let mut cont: Vec<Vec<char>> = vec![
        vec!['W', 'R', 'T', 'G'],
        vec!['W', 'V', 'S', 'M', 'P', 'H', 'C', 'G'],
        vec!['M', 'G', 'S', 'T', 'L', 'C'],
        vec!['F', 'R', 'W', 'M', 'D', 'H', 'J'],
        vec!['J', 'F', 'W', 'S', 'H', 'L', 'Q', 'P'],
        vec!['S', 'M', 'F', 'N', 'D', 'J', 'P'],
        vec!['J', 'S', 'C', 'G', 'F', 'D', 'B', 'Z'],
        vec!['B', 'T', 'R'],
        vec!['C', 'L', 'W', 'N', 'H'],
    ];
//     [W]         [J]     [J]        
//     [V]     [F] [F] [S] [S]        
//     [S] [M] [R] [W] [M] [C]        
//     [M] [G] [W] [S] [F] [G]     [C]
// [W] [P] [S] [M] [H] [N] [F]     [L]
// [R] [H] [T] [D] [L] [D] [D] [B] [W]
// [T] [C] [L] [H] [Q] [J] [B] [T] [N]
// [G] [G] [C] [J] [P] [P] [Z] [R] [H]
    cont.iter_mut().for_each(|x| x.reverse());
    let lines = utils::read_lines("./input/input_05");

    for line in lines {
        let inst = line.split(char::is_whitespace).collect::<Vec<&str>>();
        let n: u64 = inst[1].parse().unwrap();
        let from: usize = inst[3].parse().unwrap();
        let to: usize = inst[5].parse().unwrap();

        // for _ in 0..n {
        //     let c = cont[from-1].pop().unwrap();
        //     cont[to-1].push(c);
        // }

        let mut buf: Vec<char> = Vec::new();
        for _ in 0..n {
            let c = cont[from-1].pop().unwrap();
            buf.push(c);
        }
        for _ in 0..n {
            let c = buf.pop().unwrap();
            cont[to-1].push(c);
        }
    }
    let sol1 = cont.iter().map(|x| x.last().unwrap()).collect::<Vec<&char>>().iter().copied().collect::<String>();
    let sol2: u64 = 0;

    (Solution::Str(sol1), Solution::U64(sol2))
}
