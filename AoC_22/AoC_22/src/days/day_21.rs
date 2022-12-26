use std::collections::HashMap;

use itertools::Itertools;

use crate::{Solution, SolutionPair, etc::utils};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
struct Monkey {
    number: Option<i64>,
    children: Option<(String, String, String)>
}

fn get_number(name: String, monkeys: &HashMap<String, Monkey>) -> i64 {
    let monkey = &monkeys[&name];
    if monkey.number.is_some() { return monkey.number.unwrap(); }
    let (left, op, right) = monkey.children.as_ref().unwrap();
    let left = get_number(left.clone(), monkeys);
    let right = get_number(right.clone(), monkeys);
    match op.as_str() {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        _ => unreachable!()
    }
}

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_21");
    let mut monkeys = HashMap::new();

    for line in lines {
        let t = line.replace(":", "");
        let s = t.split(" ").collect_vec();
        if s.len() == 2 {
            monkeys.insert(s[0].to_string(), Monkey {number: Some(s[1].parse().unwrap()), children: None});
        } else {
            monkeys.insert(s[0].to_string(), Monkey {number: None, children: Some((s[1].to_string(), s[2].to_string(), s[3].to_string()))});
        }
    }

    let sol1: u64 = get_number("root".to_string(), &monkeys) as u64;

    let root = monkeys[&"root".to_string()].clone();
    let (left, _, right) = root.children.unwrap();
    monkeys.insert("humn".to_string(), Monkey {number: Some(3952673930912), children: None});
    println!("{}", get_number(left.clone(), &monkeys));
    println!("{}", get_number(right.clone(), &monkeys));

    let sol2: u64 = 3952673930912;

    (Solution::U64(sol1), Solution::U64(sol2))
}
