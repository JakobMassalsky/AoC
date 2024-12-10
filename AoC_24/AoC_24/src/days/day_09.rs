use itertools::Itertools;

use crate::{etc::utils::{self, SumBy}, Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

fn part1(digits: Vec<u32>) -> u64 {

    let stacklen = digits.iter().enumerate().sum_by(|(i, d)| *d * (1 - (i as u32 % 2)));
    let mut stacks = digits.iter().enumerate().filter(|(i, _)| i%2 == 0).map(|(i, l)| (i / 2, l)).collect_vec();

    let mut rearranged: Vec<(usize, u32)> = vec![];

    println!("{}", stacklen);

    let mut curlen = 0;

    let (mut id, mut back_stack) = stacks.pop().unwrap();
    let mut rem_backstack = *back_stack;

    for i in 0..digits.len() {
        if curlen >= stacklen {break}
        if i % 2 == 0 {
            rearranged.push((i/2, digits[i].min(stacklen - curlen)));
            curlen += digits[i];
        } else {
            if rem_backstack == 0 {
                (id, back_stack) = stacks.pop().unwrap();
                rem_backstack = *back_stack;
            }
            let space = digits[i].min(stacklen - curlen);
            
            if rem_backstack >= space {
                rem_backstack -= space;
                rearranged.push((id, space));
            } else {
                let mut used_space = 0;
                while used_space < space {
                    let to_use = rem_backstack.min(space-used_space);
                    used_space += to_use;
                    rearranged.push((id, to_use));
                    rem_backstack -= to_use;
                    if used_space < space {
                        assert_eq!(rem_backstack, 0);
                        (id, back_stack) = stacks.pop().unwrap();
                        rem_backstack = *back_stack;
                    }
                }
            }
            curlen += space;
        }
        // println!("{:?}", rem_backstack);

    }

    // println!("{:?}", rearranged);

    rearranged.iter().fold((0, 0), |(sum, index), (id, length)| {
        let new_sum = index * *id as u64 * *length as u64 + *length as u64 * (*length as u64).saturating_sub(1) / 2 * *id as u64;
        (sum + new_sum, index + *length as u64)
    }).0
}

fn part2(digits: Vec<u32>) -> u64 {

    // let stacklen = digits.iter().enumerate().sum_by(|(i, d)| *d * (1 - (i as u32 % 2)));
    // let mut stacks = digits.iter().enumerate().filter(|(i, _)| i%2 == 0).map(|(i, l)| (i / 2, l)).collect_vec();
    let mut absolute_indices = digits.iter().enumerate()
	.scan((0, 0, 0), |(abs_ind, _, _), (i, l)| {
        *abs_ind += *l;
		Some((*abs_ind, i, *l))
	})
	.collect_vec();

    // println!("{:?}", absolute_indices);


    for i in (0..absolute_indices.len()).rev() {
        let (pos, id, length) = absolute_indices[i];
        if id % 2 == 1 {continue;}
        if let Some((space_i, (goal_index, _, goal_length))) = absolute_indices.clone().iter().take_while(|(ind, _, _)| ind < &pos).find_position(|(_, id, le)| id % 2 == 1 && le >= &length) {
            absolute_indices[i] = (*goal_index - goal_length + length, id, length);
            absolute_indices[space_i] = (*goal_index, 1, *goal_length - length);
        }
    }

    // println!("{:?}", absolute_indices);

    absolute_indices.into_iter().filter(|(_, id, _)| id % 2 == 0).sum_by(|(end, id, length)| {
        (end as u64 - length as u64) * length as u64 * id as u64 / 2 + length as u64 * length.saturating_sub(1) as u64 / 2 * id as u64 /2
    })
}

pub fn solve() -> SolutionPair {

    let digits: Vec<u32> = utils::read_lines("./input/input_09")[0].chars().flat_map(|c| c.to_digit(10)).collect_vec();

    // Your solution here...
    let sol1: u64 = part1(digits.clone());
    let sol2: u64 = part2(digits);

    (Solution::U64(sol1), Solution::U64(sol2))
}
