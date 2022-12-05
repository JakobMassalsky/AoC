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
        let inst = line
            .split(' ')
            .map(|x| x.parse::<usize>().unwrap_or(0))
            .collect::<Vec<usize>>();

        let i = cont[inst[3]-1].len() - inst[1];
        let c = cont[inst[3]-1].split_off(i);
        cont[inst[5]-1].extend(&mut c.iter().rev()); 

        let i = cont2[inst[3]-1].len() - inst[1];
        let mut c = cont2[inst[3]-1].split_off(i);
        cont2[inst[5]-1].append(&mut c);
    }

    let sol1 = cont.iter().map(|x| x.last().unwrap()).collect::<String>();
    let sol2 = cont2.iter().map(|x| x.last().unwrap()).collect::<String>();

    (Solution::Str(sol1), Solution::Str(sol2))
}
