use std::{borrow::BorrowMut, cell::RefCell, collections::{HashMap, HashSet}, rc::Rc, u64};

use itertools::Itertools;
use priority_queue::PriorityQueue;

use crate::{Solution, SolutionPair, etc::utils};

///////////////////////////////////////////////////////////////////////////////

fn get_neighbours((loc, dir): ((isize, isize), (isize, isize))) -> [((isize, isize), (isize, isize), u64); 3] {
    [((loc.0 + dir.0, loc.1 + dir.1), dir, 1),
        (loc, (dir.1, dir.0), 1000),
        (loc, (-dir.1, -dir.0), 1000)]
}

#[derive(Debug, Clone)]
struct PathTree {
    item: (isize, isize),
    branches: Vec<Rc<RefCell<PathTree>>>,
}

impl PathTree {
    fn get_path(&self) -> HashSet<(isize, isize)> {
        let mut path = HashSet::from([self.item]);
        for b in self.branches.iter() {
            let b_ref = b.borrow();
            path.extend(b_ref.get_path());
        }
        path
    }
}

fn dijkstra(map: &Vec<Vec<char>>, start: ((isize, isize), (isize, isize))) -> (u64, u64) {
    
    let mut q = PriorityQueue::new();
    q.push(start, u64::MAX);
    let mut visited = HashMap::new();
    visited.insert(start, u64::MAX);
    let mut paths = HashMap::new();
    paths.insert(start, PathTree {item: start.0, branches: vec![]});
    let mut min_found = 0;

    while let Some((node, prio)) = q.pop() {
        for (nloc, ndir, cost) in get_neighbours(node) {
            let nc = prio - cost;
            if nc < min_found {continue}
            match map[nloc.0 as usize][nloc.1 as usize] {
                '#' => (),
                _ => {
                    if node.1 != ndir && map[(nloc.0 + ndir.0) as usize][(nloc.1 + ndir.1) as usize] == '#' {
                        continue;
                    }
                    if let Some(&vc) = visited.get(&(nloc, ndir)) {
                        if vc == nc {
                            let branch = paths.get(&node).unwrap().clone(); // Assume this is `Rc<RefCell<PathTree>>`
                            let cur_path = paths.get_mut(&(nloc, ndir)).unwrap();
                            cur_path.borrow_mut().branches.push(Rc::new(RefCell::new(branch)));
                        }
                        if vc >= nc {continue}
                    }
                    if map[nloc.0 as usize][nloc.1 as usize] == 'E' {
                        min_found = nc;
                    }
                    q.push((nloc, ndir), nc);
                    visited.insert((nloc, ndir), nc);
                    paths.insert((nloc, ndir), PathTree {item: nloc, branches: vec![Rc::new(RefCell::from(paths.get(&node).unwrap().clone()))]});
                }
            }
        }
    }
    (u64::MAX - visited[&((1, map[1].len() as isize - 2), (0, 1))], paths[&((1, map[1].len() as isize - 2), (0, 1))].get_path().len() as u64)
}

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_16")
        .into_iter().map(|s| s.chars().collect_vec())
        .collect_vec();

    // Your solution here...
    let (sol1, sol2) = dijkstra(&lines, ((lines.len() as isize - 2, 1), (0, 1)));

    (Solution::U64(sol1), Solution::U64(sol2))
}
