use itertools::Itertools;

use crate::{Solution, SolutionPair, etc::utils};

///////////////////////////////////////////////////////////////////////////////

struct Monkey {
    items: Vec<u64>,
    op: bool,
    n2: u64,
    test: u64,
    on_success: u8,
    on_fail: u8,
    count: u64
}

fn gen_monkeys() -> Vec<Monkey> {
    let inpt = include_str!("../../input/input_11");
    let mut monkeys: Vec<Monkey> = Vec::new();
    for monkey in inpt.split("\r\n\r\n") {
        let prop = monkey.to_string()
            .split("\r\n")
            .map(|x| x.trim().replace(":", "").replace(",", ""))
            .collect_vec();
        let m = Monkey {
            items: prop[1].split(" ").skip(2).map(|x| x.parse::<u64>().unwrap()).collect_vec(),
            op: prop[2].split(" ").collect_vec()[4] == "+",
            n2: prop[2].split(" ").collect_vec()[5].parse::<u64>().unwrap_or(0),
            test: prop[3].split(" ").last().unwrap().parse().unwrap(),
            on_success: prop[4].split(" ").last().unwrap().parse().unwrap(),
            on_fail: prop[5].split(" ").last().unwrap().parse().unwrap(),
            count: 0,
        };
        monkeys.push(m);
    }
    monkeys
}

fn monkey_business(monkeys: &mut Vec<Monkey>, rounds: u64, max: u64) -> u64 {
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let mut items_buf: Vec<u64> = Vec::new();
            let mut index_buf: Vec<usize> = Vec::new();
            let m = &monkeys[i];
            for item in &m.items {
                let mut n = if m.n2 == 0 { *item } else { m.n2 };
                n = if m.op {*item + n} else {*item * n};
                n = if max == 0 {n / 3} else {n % max};
                items_buf.push(n);
                index_buf.push(if n % m.test == 0 
                    { m.on_success as usize } 
                    else { m.on_fail as usize });
            }
            for (item, index) in items_buf.iter().zip_eq(index_buf.iter()) {
                monkeys[*index].items.push(*item);
            }
            monkeys[i].count += monkeys[i].items.len() as u64;
            monkeys[i].items.clear();
        }
    }
    monkeys.iter().map(|x| x.count).sorted().rev().take(2).reduce(|a, b| a*b).unwrap()
}

pub fn solve() -> SolutionPair {
    
    let sol1 = monkey_business(&mut gen_monkeys(), 20, 0);

    let mut monkeys = gen_monkeys();
    let max = monkeys.iter().map(|x| x.test).reduce(|a, b| a*b).unwrap();
    let sol2 = monkey_business(&mut monkeys, 10000, max);

    (Solution::U64(sol1), Solution::U64(sol2))
}
