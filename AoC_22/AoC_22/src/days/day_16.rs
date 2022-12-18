use std::collections::{HashSet, HashMap, VecDeque};

use itertools::Itertools;
use ndarray::{Array2, Array};

use crate::{Solution, SolutionPair, etc::utils};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_16");
    let lines = lines.iter().map(|s| s.replace(",", "")).collect_vec();
    let valve_values = lines.iter().map(|s| utils::extract_ints::<usize>(s, &[])[0]).collect_vec();
    let chambers = lines.iter().map(|s| s.split(" ").collect_vec()).collect_vec();

    let mut chamber_to_index = HashMap::new();
    for (i, chamber) in chambers.iter().enumerate() {
        chamber_to_index.insert(chamber[1], i);
    }
    let mut indices_of_interest = valve_values.iter().enumerate().filter(|(i, &v)| v > 0 || i == &chamber_to_index["AA"]).map(|(i, _)| i).collect_vec();
    let mut adjacency_matrix: Array2<u64> = Array2::zeros([chambers.len(), chambers.len()]);
    let mut paths = Array2::zeros([chambers.len(), chambers.len()]);
    for (i, chamber) in chambers.iter().enumerate() {
        for &n in chamber[9..].iter() {
            adjacency_matrix[[i, chamber_to_index[n]]] = 1;
            adjacency_matrix[[chamber_to_index[n], i]] = 1;
        }
    }

    // println!("{:?}", connections);
    let mut adjacency_matrix_acc: Array2<u64> = adjacency_matrix.clone();
    for i in 1..30 {
        for (&x, &y) in indices_of_interest.iter().tuple_combinations() {
            if adjacency_matrix_acc[[x, y]] == 0 { continue; }
            if paths[[x, y]] != 0 { continue; }
            paths[[x, y]] = i;
            paths[[y, x]] = i;
        }
        adjacency_matrix_acc = adjacency_matrix_acc.dot(&adjacency_matrix);
    }

    // Matrix AA AA = 0
    println!("{:?}", paths);
    println!("starting search");
    let start = chamber_to_index["AA"];
    indices_of_interest.remove(indices_of_interest.iter().position(|&p| p==start).unwrap());
    let score = bfs(start, &paths, &valve_values, &indices_of_interest);
    // let scores = bfs2("AA".to_string(), &connections, &valves);
    // Your solution here...
    let sol1: u64 = score;
    let sol2: u64 = 0;

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn get_neighbours(pos: usize, paths: &Array2<i32>, indices_of_interest: &Vec<usize>) -> Vec<[usize; 2]> {
    return indices_of_interest.iter().filter(|&&x| x != pos).map(|&chamber| [chamber, paths[[pos, chamber]] as usize]).collect_vec();
}

fn bfs(start: usize, paths: &Array2<i32>, valves: &Vec<usize>, indices_of_interest: &Vec<usize>) -> u64 {
    
    let mut vec = vec![];
    let mut visit_queue = VecDeque::from_iter([(start.clone(), indices_of_interest.clone(), 0, 0)]);
    let mut visited = HashMap::new();
    visited.insert((start, indices_of_interest.clone()), [0, 0]);

    while !visit_queue.is_empty() {
        let (pos, remaining_valves, timestep, score) = visit_queue.pop_front().unwrap();
        visited.insert((pos, remaining_valves.clone()), [timestep, score]);
        let neighbours = get_neighbours(pos, paths, &remaining_valves.clone());
        for [new_pos, dist] in neighbours {
            let new_t = timestep + dist + 1;
            if new_t >= 30 { 
                vec.push(score);
                continue; 
            }
            let new_score = score + valves[pos]*(30-new_t);
            let mut v = remaining_valves.clone();
            v.remove(remaining_valves.iter().position(|&p| p==new_pos).unwrap());
            if visited.contains_key(&(new_pos, v.clone()))  {
                let [t, s] = visited[&(new_pos, v.clone())];
                if t <= new_t && s >= new_score {
                    continue;
                }
            }
            visit_queue.push_back((new_pos, v, new_t, new_score));
        }
    }
    // println!("{}", vec.iter().max().unwrap());
    *visited.values().map(|[_, s]| s).max().unwrap() as u64
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
