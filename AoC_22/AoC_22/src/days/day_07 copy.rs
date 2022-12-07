use std::collections::HashMap;

use crate::{Solution, SolutionPair, etc::utils};

///////////////////////////////////////////////////////////////////////////////

struct Dir<'a> {
    name: String,
    parent: Option<&'a mut Dir<'a>>,
    children: HashMap<String, &'a mut Dir<'a>>,
    files: Vec<u64>
}

impl Default for Dir<'_> {
    fn default() -> Dir<'a> {
        Dir {
            name: "".to_string(), 
            parent: None, 
            children: HashMap::new(), 
            files: Vec::new()
        }
    }
}

pub fn solve<'a>() -> SolutionPair {

    let lines = utils::read_lines("./input/input_07");

    let mut cd: &mut Dir<'a> = &mut Dir::<'a> {
        name: "".to_string(), 
        parent: None, 
        children: HashMap::new(), 
        files: Vec::new()};

    for line in lines {
        if line.starts_with("$ cd") {
            if line.contains("..") {
                cd = cd.parent.unwrap();
            } else {
                let name = line.split(" ").last().unwrap();
                cd = cd.children[name];
            }
        }
        if line.starts_with("dir") {
            let name = line.split(" ").last().unwrap();
            let &mut child = &mut Dir::default();
            cd.children.insert(name.to_string(), &mut child);
        }
    }


    // Your solution here...
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::U64(sol1), Solution::U64(sol2))
}
