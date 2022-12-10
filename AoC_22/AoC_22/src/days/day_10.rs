use crate::{Solution, SolutionPair, etc::utils};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_10");
    let mut x: i32 = 1;
    let mut c: i32 = 1;
    let mut p = 0;
    let mut sol1 = 0;
    let mut add = 0;
    let mut screen = ['.'; 240];
    while p < lines.len() {
        if (c-20) % 40 == 0 { 
            sol1 += c * x;
        }
        let cp = (c-1) % 40;
        if cp == x || cp == x-1 || cp == x+1 {
            screen[(c-1) as usize] = '#';
        }
        let op = &lines[p];
        p += 1;
        if op.starts_with("a") {
            if add != 1 {
                add += 1;
                p -= 1;
            } else {
                x += op.split(' ').last().unwrap().parse::<i32>().unwrap();
                add = 0;
            }
        }
        c += 1;
    }

    let sol2 = screen
        .chunks(40)
        .map(|line| line
            .iter()
            .collect::<String>()+"\n")
        .collect::<String>();

    println!("{}", sol2);

    (Solution::I32(sol1), Solution::Str(sol2))
}
