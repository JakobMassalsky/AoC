use std::collections::{LinkedList, HashSet, VecDeque};

use itertools::Itertools;

use crate::{Solution, SolutionPair, etc::utils};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_20");
    let list = lines.iter().map(|i| i.parse::<isize>().unwrap() * 811589153).collect_vec();
    let mut ring_list = Vec::from_iter(list.iter().cloned().enumerate());

    let l = list.len() as isize;
    // let mut i: usize = 0;
    // while i < list.len() {
    //     let (ele_index, ele_value) = ring_list[i as usize];
    //     let mut new_i = (ele_index as isize + ele_value) % (l-1);
    //     if new_i < 0 {new_i += l-1;}
    //     ring_list[i as usize].0 = new_i as usize;
    //     i += 1;
    //     for v ring_list.iter_mut()
    // }

    println!("{}", -10 % 3);
    for _ in 0..10 {    
        let mut i: isize = 0;
        let mut ii: usize = 0;
        while ii < list.len() {
            let ele = ring_list[i as usize];
            if ele.0 != ii {
                i += 1;
                i %= l;
            } else {
                ring_list.remove(i as usize);
                let mut v = (i + ele.1) % (l-1);
                if v < 0 {v += l-1;}
                ring_list.insert(v as usize, ele);
                ii += 1;
            }
        }
    }

    let zero = ring_list.iter().position(|v| v.1==0).unwrap();
    // Your solution here...
    let sol1: i64 = (ring_list[(1000+zero)%l as usize].1 + ring_list[(2000+zero)%l as usize].1 + ring_list[(3000+zero)%l as usize].1) as i64;
    let sol2: u64 = 0;

    (Solution::I64(sol1), Solution::U64(sol2))
}
