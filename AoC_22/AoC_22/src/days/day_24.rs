use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

use crate::{Solution, SolutionPair, etc::utils};

///////////////////////////////////////////////////////////////////////////////

#[derive(Hash, Eq, PartialEq, Debug)]
struct Blizzard {
    x: isize,
    y: isize,
    dir: char
}

impl Blizzard {
    fn do_movement(&mut self, (xmin, xmax, ymin, ymax): (isize, isize, isize, isize)) {
        self.x += match self.dir {
            '<' => -1,
            '>' => 1,
            _ => 0
        };
        if self.x < xmin { self.x = xmax; }
        if self.x > xmax { self.x = xmin; }
        self.y += match self.dir {
            '^' => -1,
            'v' => 1,
            _ => 0
        };
        if self.y < ymin { self.y = ymax; }
        if self.y > ymax { self.y = ymin; }
    }
}

fn get_blizzards(blizzards: &Vec<Blizzard>) -> HashSet<[isize; 2]> {
    HashSet::<_>::from_iter(blizzards.iter().map(|b| [b.x, b.y]))
}

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_24");
    let b_chars = HashSet::<_>::from_iter(['<', '>', 'v', '^']);
    let mut blizzards = vec![];

    for (y, line) in lines.iter().enumerate() {
        for (x, dir) in line.char_indices() {
            if b_chars.contains(&dir) {
                blizzards.push(Blizzard {x: x as isize, y: y as isize, dir});
            }
        }
    }
    let bounds = (1, (lines[0].len()-2) as isize, 1, (lines.len()-2) as isize);
    let goal = ((lines[0].len()-2) as isize, (lines.len()-1) as isize);
    blizzards.iter_mut().for_each(|b| b.do_movement(bounds));
    let (path1, b1) = bfs((1, 0), &mut blizzards, goal, bounds);
    println!("{path1}");
    let (path2, b2) = bfs(goal, b1, (1, 0), bounds);
    println!("{path2}");
    let (path3, _) = bfs((1, 0), b2, goal, bounds);
    println!("{path3}");

    // Your solution here...
    let sol1: u64 = (path1 + path2 + path3 - 2) as u64;
    let sol2: u64 = 0;

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn get_neighbours(x: isize, y: isize, blizzards: &HashSet<[isize; 2]>, bounds: (isize, isize, isize, isize), start: [isize; 2], goal: [isize; 2]) -> Vec<[isize; 2]> {
    [[x, y], [x+1, y], [x-1, y], [x, y+1], [x, y-1]].into_iter()
        .filter(|&[x, y]| [x, y] == start || [x, y] == goal || (x >= bounds.0 && x <= bounds.1 && y >= bounds.2 && y <= bounds.3))
        .filter(|loc| !blizzards.contains(loc)).collect_vec()
}

fn bfs(start: (isize, isize), blizzards: &mut Vec<Blizzard>, goal: (isize, isize), bounds: (isize, isize, isize, isize)) -> (isize, &mut Vec<Blizzard>) {
    println!("{bounds:?}, {goal:?}");
    let mut last_time = 0;
    let mut queue = VecDeque::from_iter([[start.0, start.1, last_time]]);
    let mut visited = HashSet::<_>::from_iter([[start.0, start.1, last_time]]);
    let mut blizzard_locations = get_blizzards(blizzards);

    while !queue.is_empty() {
        let [x, y, time] = queue.pop_front().unwrap();

        if time != last_time {
            last_time = time;
            blizzards.iter_mut().for_each(|b| b.do_movement(bounds));
            blizzard_locations = get_blizzards(blizzards);
            visited.clear();
        }

        for [nx, ny] in get_neighbours(x, y, &blizzard_locations, bounds, [start.0, start.1], [goal.0, goal.1]) {
            if time < 2 {
                // println!("{:?}", [nx, ny, time+1])
            } else if time > 10000 {
                break;
            }
            let node = [nx, ny, time+1];
            if (nx, ny) == goal {return (time+1, blizzards)};
            if visited.contains(&node) { continue; }
            queue.push_back(node);
            visited.insert(node);
        }
    }
    (-1, blizzards)
}
