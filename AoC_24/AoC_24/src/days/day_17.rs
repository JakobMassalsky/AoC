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

fn do_op(opcode: u64, operand: u64, reg: &mut Registers, p: bool) {
    reg.ip += 2;
    match opcode {
        0 => reg.A = reg.A / 2_u64.pow(get_combo(operand, reg) as u32),
        1 => reg.B = reg.B ^ operand,
        2 => reg.B = get_combo(operand, reg) % 8,
        3 => if reg.A != 0 {reg.ip = operand},
        4 => reg.B = reg.B ^ reg.C,
        5 => if p {print!("{},", get_combo(operand, reg) % 8)},
        6 => reg.B = reg.A / 2_u64.pow(get_combo(operand, reg) as u32),
        7 => reg.C = reg.A / 2_u64.pow(get_combo(operand, reg) as u32),
        _ => unreachable!()
    };
}

fn find_bits(instructions: &Vec<u64>, index: i64, total: u64) -> Option<u64> {
    if index < 0 {return Some(total)}
    for a in 0..8 {
        let new_a = (total << 3) + a;
        let mut reg = Registers {ip: 0, A: new_a, B: 0, C: 0};
        while reg.ip < instructions.len() as u64 - 2 {
            do_op(instructions[reg.ip as usize], instructions[reg.ip as usize + 1], &mut reg, false);
        }
        if reg.B % 8 == instructions[index as usize] {
            if let Some(result) = find_bits(instructions, index-1, new_a) {
                return Some(result);
            }
        }
    }
    None
}

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_17");
    let (regs, ins) = lines.split(|s| s == "").next_tuple().unwrap();
    let registers: Vec<Vec<u64>> = regs.into_iter().map(|s| extract_ints(s, &[])).collect_vec();

    let mut computer = Registers {ip: 0, A: registers[0][0], B: registers[1][0], C: registers[2][0]};
    let instructions: Vec<u64> = ins[0].split(" ").skip(1).next().unwrap().split(",").parse_all().collect();

    while computer.ip < instructions.len() as u64 {
        do_op(instructions[computer.ip as usize], instructions[computer.ip as usize + 1], &mut computer, true);
    }
    let res = find_bits(&instructions, instructions.len() as i64 - 1, 0);

    // Your solution here...
    let sol1: u64 = 0;
    let sol2: u64 = res.unwrap();

    (Solution::U64(sol1), Solution::U64(sol2))
}
