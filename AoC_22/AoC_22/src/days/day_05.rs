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
    
    cont.iter_mut().for_each(|x| x.reverse());
    let mut cont2 = cont.to_vec();
    let lines = utils::read_lines("./input/input_05");

    for line in lines {
        let inst = line.split(char::is_whitespace).collect::<Vec<&str>>();
        let n: usize = inst[1].parse().unwrap();
        let from: usize = inst[3].parse().unwrap();
        let to: usize = inst[5].parse().unwrap();

        let i = cont[from-1].len() - n;
        let c = cont[from-1].split_off(i);
        cont[to-1].extend(&mut c.iter().rev());

        let i = cont2[from-1].len() - n;
        let mut c = cont2[from-1].split_off(i);
        cont2[to-1].append(&mut c);
    }

    let sol1 = cont.iter().map(|x| x.last().unwrap()).collect::<Vec<&char>>().iter().copied().collect::<String>();
    let sol2 = cont2.iter().map(|x| x.last().unwrap()).collect::<Vec<&char>>().iter().copied().collect::<String>();

    (Solution::Str(sol1), Solution::Str(sol2))
}
