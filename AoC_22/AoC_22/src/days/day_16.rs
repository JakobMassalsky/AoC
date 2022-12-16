use std::collections::{HashSet, HashMap, VecDeque};

use itertools::Itertools;

use crate::{Solution, SolutionPair, etc::utils};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_16");
    let lines = lines.iter().map(|s| s.replace(",", "")).collect_vec();
    let valve_values = lines.iter().map(|s| utils::extract_ints::<u64>(s, &[])[0]).collect_vec();

    let chambers = lines.iter().map(|s| s.split(" ").collect_vec()).collect_vec();

    let mut connections = HashMap::new();
    let mut valves = HashMap::new();
    let mut activated = HashMap::new();
    for (i, chamber) in chambers.iter().enumerate() {
        let paths = chamber[9..].iter().cloned().map(|x| (x, 1)).collect_vec();
        for (n, c) in paths {
            connections.insert([chamber[1], n], c);
            connections.insert([n, chamber[1]], c);
        }
        valves.insert(chamber[1].to_string(), valve_values[i]);
        activated.insert(chamber[1], false);
    }

    // println!("{:?}", connections);
    for chamber in chambers {
        let name = chamber[1];
        if valves[name] == 0 && name != "AA" {
            let adjacents1 = connections.iter().filter(|(&[k1, _], _)| k1 == name).map(|(&[_, k2], v)| (k2.clone(), v.clone())).collect_vec();
            // let mut adjacents2 = connections.iter().filter(|(&[_, k2], _)| k2 == name).map(|(&[k1, _], v)| (k1.clone(), v.clone())).collect_vec();
            // adjacents1.append(&mut adjacents2);
            for ((name1, v1), (name2, v2)) in adjacents1.iter().tuple_combinations() {
                if name1 == name2 {continue;}
                if !connections.contains_key(&[*name1, *name2]) || connections[&[*name1, *name2]] > v1+v2 {
                    connections.insert([name1, name2], v1+v2);
                    connections.insert([name2, name1], v1+v2);
                }
            } 
            let keys = connections.keys().cloned().collect_vec();
            for [k1, k2] in keys {
                if k1 == name || k2 == name {
                    connections.remove(&[k1, k2]);
                }
            }
        }
    }

    let scores = bfs("AA".to_string(), &connections, &valves);
    // let scores = bfs2("AA".to_string(), &connections, &valves);
    println!("{:?}", scores.iter().max()); // VM: 1979
    // Your solution here...
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn get_neighbours(pos: (String, HashSet<String>), connections: &HashMap<[&str; 2], u64>, valve: &HashMap<String, u64>) -> Vec<((String, HashSet<String>), u64, u64)> {
    let mut neighbours = Vec::new();
    for (&[k1, k2], v) in connections {
        if k1 == pos.0 {
            let nh = pos.1.clone();//HashSet::new().union(&pos.1).cloned().collect();
            neighbours.push(((k2.to_string(), nh), *v, 0));
        }
    }
    if !pos.1.contains(&pos.0) {
        let mut h = HashSet::new().union(&pos.1).cloned().collect::<HashSet<String>>();
        h.insert(pos.0.clone());
        neighbours.push(((pos.0.clone(), h), 1, valve[&pos.0]));
    }
    return neighbours;
}

fn bfs(start: String, connections:  &HashMap<[&str; 2], u64>, valves: &HashMap<String, u64>) -> Vec<u64> {
    
    let mut score_at_end = Vec::new();
    let mut visit_queue = VecDeque::from_iter([(start.clone(), 0, HashSet::new(), 0)]);
    let mut visited = HashMap::new();
    visited.insert((start.clone(), vec!["".to_string()]), 0);

    while !visit_queue.is_empty() {
        let (p, t, v, s) = visit_queue.pop_front().unwrap();
        let neighbours = get_neighbours((p, v), &connections, &valves);
        for ((name, opened), dt, score) in neighbours {
            score_at_end.push(s);
            let new_t = t+dt;
            if new_t < 30 {
                let new_score = s + score*(30-new_t);
                let op = opened.iter().cloned().sorted().collect_vec();
                if !visited.contains_key(&(name.clone(), op.clone())) || visited[&(name.clone(), op.clone())] < new_score {
                    visit_queue.push_back((name.clone(), new_t, opened, new_score));
                    visited.insert((name, op), new_score);
                }
            }
        }
    }
    // println!("{:?}", score_at_end); 
    score_at_end
}

// fn bfs2(start: String, connections:  &HashMap<[&str; 2], u64>, valves: &HashMap<String, u64>) -> Vec<u64> {
    
//     let mut score_at_end = Vec::new();
//     let mut visit_queue = VecDeque::from_iter([(start.clone(), start.clone(), 0, HashSet::new(), 0)]);
//     let mut visited = HashMap::new();
//     visited.insert((start.clone(), start.clone(), vec!["".to_string()]), 0);

//     while !visit_queue.is_empty() {
//         let (py, pe, t, v, s) = visit_queue.pop_front().unwrap();
//         let neighbours = get_neighbours_new((py, pe, v), &connections, &valves);
//         for ((namey, namee, opened), dt, score) in neighbours {
//             score_at_end.push(s);
//             let new_t = t+dt;
//             if new_t < 26 {
//                 let new_score = s + score*(26-new_t);
//                 let op = opened.iter().cloned().sorted().collect_vec();
//                 if !visited.contains_key(&(namey.clone(), namee.clone(), op.clone())) || visited[&(namey.clone(), namee.clone(), op.clone())] < new_score {
//                     visit_queue.push_back((namey.clone(), namee.clone(), new_t, opened, new_score));
//                     visited.insert((namey, namee, op), new_score);
//                 }
//             }
//         }
//     }
//     // println!("{:?}", score_at_end); 
//     score_at_end
// }
