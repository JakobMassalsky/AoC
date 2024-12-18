use itertools::Itertools;

use crate::{etc::utils::{self, extract_ints, ParseAll}, Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone)]
struct Registers {
    ip: u64,
    A: u64,
    B: u64,
    C: u64,
}

fn get_combo(operand: u64, reg: &Registers) -> u64 {
    match operand {
        0..=3 => operand,
        4 => reg.A,
        5 => reg.B,
        6 => reg.C,
        _ => unreachable!(),
    }
}

fn do_op(opcode: u64, operand: u64, reg: &mut Registers) {
    reg.ip += 2;
    match opcode {
        0 => reg.A = reg.A / 2_u64.pow(get_combo(operand, reg) as u32),
        1 => reg.B = reg.B ^ operand,
        2 => reg.B = get_combo(operand, reg) % 8,
        3 => if reg.A != 0 {reg.ip = operand},
        4 => reg.B = reg.B ^ reg.C,
        5 => (),//print!("{},", get_combo(operand, reg) % 8),
        6 => reg.B = reg.A / 2_u64.pow(get_combo(operand, reg) as u32),
        7 => reg.C = reg.A / 2_u64.pow(get_combo(operand, reg) as u32),
        _ => unreachable!()
    };
}

// fn part2(instructions: Vec<u64>, r: Registers) {
//     // for a in 4304721..=43046722 {
//         let mut reg = r.clone();
//         reg.A = 281474976710650;
//         while reg.ip < instructions.len() as u64 {
//             do_op(instructions[reg.ip as usize], instructions[reg.ip as usize + 1], &mut reg);
//         }
//         // for _ in 0..7 {
//         //     do_op(instructions[reg.ip as usize], instructions[reg.ip as usize + 1], &mut reg);
//         // }
//         // if reg.B % 8 == 2 {
//         //     println!("{}", a);

//         // }

//     // }
//     // 1,7,6,5,1,0,5,0,7
//     // 2,4,1,3,7,5,4,2,0,3,1,5,5,5,3,0
// }

fn find_bits(instructions: &Vec<u64>, r: Registers, ins: i64, tota: u64) -> Option<u64> {
    if ins < 0 {return Some(tota)}
    let mut options = vec![];
    let c = instructions[ins as usize];
    for a in 0..8 {
        let mut reg = r.clone();
        reg.ip = 0;
        reg.A = (tota << 3) + a;
        while reg.ip < instructions.len() as u64 {
            do_op(instructions[reg.ip as usize], instructions[reg.ip as usize + 1], &mut reg);
        }
        if reg.B % 8 == c {
            options.push(find_bits(instructions, reg, ins-1, (tota << 3) + a));
        }
        // println!("{}, {}", a, reg.B % 8);
    }
    
    options.into_iter().find(|x| x.is_some()).unwrap_or(None)
}

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_17");
    let (regs, ins) = lines.split(|s| s == "").next_tuple().unwrap();
    let registers: Vec<Vec<u64>> = regs.into_iter().map(|s| extract_ints(s, &[])).collect_vec();

    let mut computer = Registers {ip: 0, A: registers[0][0], B: registers[1][0], C: registers[2][0]};//Registers {ip: 0, A: 10, B: 0, C: 0};
    let instructions: Vec<u64> = ins[0].split(" ").skip(1).next().unwrap().split(",").parse_all().collect();
    // let instructions: Vec<u64> = "5,0,5,1,5,4".split(",").parse_all().collect();
    // println!("{:?}", computer);

    // while computer.ip < instructions.len() as u64 {
    //     do_op(instructions[computer.ip as usize], instructions[computer.ip as usize + 1], &mut computer);
    // }
    let res = find_bits(&instructions, computer, instructions.len() as i64 - 1, 0);
    println!("{}", res.unwrap());

    // Your solution here...
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::U64(sol1), Solution::U64(sol2))
}
