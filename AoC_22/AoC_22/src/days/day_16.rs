use std::collections::{HashSet, HashMap, VecDeque};

use itertools::Itertools;
use ndarray::{Array2, Array, s};

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
    let indices_of_interest = valve_values.iter().enumerate().filter(|(i, &v)| v > 0 || i == &chamber_to_index["AA"]).map(|(i, _)| i).collect_vec();
    let mut adjacency_matrix: Array2<u64> = Array2::zeros([chambers.len(), chambers.len()]);
    let mut paths = Array2::zeros([chambers.len(), chambers.len()]);
    for (i, chamber) in chambers.iter().enumerate() {
        for &n in chamber[9..].iter() {
            adjacency_matrix[[i, chamber_to_index[n]]] = 1;
            adjacency_matrix[[chamber_to_index[n], i]] = 1;
        }
    }

    // println!("{:?}", chamber_to_index);
    let mut adjacency_matrix_acc: Array2<u64> = adjacency_matrix.clone();
    for i in 1..30 {
        for (x, y) in (0..adjacency_matrix.raw_dim()[0]).tuple_combinations() {
            if adjacency_matrix_acc[[x, y]] == 0 { continue; }
            if paths[[x, y]] != 0 { continue; }
            paths[[x, y]] = i;
            paths[[y, x]] = i;
        }
        adjacency_matrix_acc = adjacency_matrix_acc.dot(&adjacency_matrix);
    }

    // Matrix AA AA = 0
    // println!("{:?}", paths);
    println!("starting search");
    let start = chamber_to_index["AA"];
    // let score = bfs(start, &paths, &valve_values, &indices_of_interest);
    let score2 = bfs2(start, &paths, &valve_values, &indices_of_interest);
    // Your solution here...
    let sol1: u64 = 0; //score;
    let sol2: u64 = score2;

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn remove_chamber(chamber: usize, indices_of_interest: &Vec<usize>) -> Vec<usize> {
    let mut v = indices_of_interest.clone();
    v.remove(indices_of_interest.iter().position(|&p| p==chamber).unwrap());
    v
}

fn try_remove_chamber(chamber: usize, indices_of_interest: &Vec<usize>) -> Vec<usize> {
    if !indices_of_interest.contains(&chamber) {
        return indices_of_interest.clone();
    }
    let mut v = indices_of_interest.clone();
    v.remove(indices_of_interest.iter().position(|&p| p==chamber).unwrap());
    v
}

fn get_neighbours(pos: usize, paths: &Array2<i32>, indices_of_interest: &Vec<usize>) -> Vec<(usize, usize, Vec<usize>)> {
    return indices_of_interest.iter().map(|&chamber| (chamber, paths[[pos, chamber]] as usize, remove_chamber(chamber, &indices_of_interest))).collect_vec();
}

fn bfs(start: usize, paths: &Array2<i32>, valves: &Vec<usize>, remaining_valves: &Vec<usize>) -> u64 {
    
    let remaining_valves = remove_chamber(start, remaining_valves);

    let node = ((start, remaining_valves.clone()), [0, 0]);
    let mut visit_queue = VecDeque::from_iter([node.clone()]);
    let mut visited = HashMap::new();
    visited.insert(node.0, node.1);

    while !visit_queue.is_empty() {
        let ((pos, new_remaining_valves), [timestep, score]) = visit_queue.pop_front().unwrap();
        let neighbours = get_neighbours(pos, paths, &new_remaining_valves);

        for (new_pos, dist, remaining) in neighbours {
            let new_t = timestep + dist + 1;
            if new_t >= 30 { 
                continue; 
            }
            let new_score = score + valves[new_pos]*(30-new_t);
            if visited.contains_key(&(new_pos, remaining.clone())) && visited[&(new_pos, remaining.clone())][1] >= new_score { //  
                continue;
            }
            visit_queue.push_back(((new_pos, remaining.clone()), [new_t, new_score]));
            visited.insert((new_pos, remaining.clone()), [new_t, new_score]);
        }
    }
    
    *visited.values().map(|[_, s]| s).max().unwrap() as u64
}

// fn get_neighbours2(pos1: usize, pos2: usize, time_to_arrive1: usize, time_to_arrive2: usize, paths: &Array2<i32>, remaining_valves: &Vec<usize>, valve_values: &Vec<usize>) -> Vec<(usize, usize, usize, usize, usize, usize, Vec<usize>)> {
//     if time_to_arrive1 != 0 && time_to_arrive2 != 0 {
//         return vec![(pos1, pos2, time_to_arrive1-1, time_to_arrive2-1, 0, 0, remaining_valves.clone())];
//     }
//     if time_to_arrive1 == 0 && time_to_arrive2 != 0 {
//         return remaining_valves.iter().map(|&chamber| (chamber, pos2, paths[[pos1, chamber]] as usize + 1, time_to_arrive2-1, valve_values[chamber], 0, remove_chamber(chamber, &remaining_valves))).collect_vec();
//     }
//     if time_to_arrive1 != 0 && time_to_arrive2 == 0 {
//         return remaining_valves.iter().map(|&chamber| (pos1, chamber, time_to_arrive1-1, paths[[pos2, chamber]] as usize + 1, 0, valve_values[chamber], remove_chamber(chamber, &remaining_valves))).collect_vec();
//     }
//     remaining_valves
//         .iter()
//         .tuple_combinations()
//         .map(|(&chamber1, &chamber2)| (chamber1, 
//             chamber2, 
//             paths[[pos1, chamber1]] as usize + 1, 
//             paths[[pos2, chamber2]] as usize + 1, 
//             valve_values[chamber1],
//             valve_values[chamber2],
//             remove_chamber(chamber2, &remove_chamber(chamber1, &remaining_valves))))
//         .collect_vec()
// }

// fn bfs2(start: usize, paths: &Array2<i32>, valves: &Vec<usize>, remaining_valves: &Vec<usize>) -> u64 {
    
//     let remaining_valves = remove_chamber(start, remaining_valves);

//     let node = ((start, start, 0, 0, remaining_valves.clone()), [0, 0]);
//     let mut visit_queue = VecDeque::from_iter([node.clone()]);
//     let mut visited = HashMap::new();
//     visited.insert((start, start, remaining_valves.clone()), node.1);
//     let mut max = 0;

//     while !visit_queue.is_empty() {
//         let ((pos1, pos2, steps_remaining1, steps_remaining2, new_remaining_valves), [timestep, score]) = visit_queue.pop_front().unwrap();
//         let neighbours = get_neighbours2(pos1, pos2, steps_remaining1, steps_remaining2, paths, &new_remaining_valves, &valves);

//         for (new_pos1, new_pos2, toa1, toa2, score_gain1, score_gain2, remaining) in neighbours {
//             let new_t = timestep+1;
//             if timestep+toa1 >= 26 || timestep+toa2 >= 26 { 
//                 continue; 
//             }
//             let new_score = score + score_gain1*(26-new_t-toa1) + score_gain2*(26-new_t-toa2);
//             max = new_score.max(max);
//             if visited.contains_key(&(new_pos1, new_pos2, remaining.clone())) && visited[&(new_pos1, new_pos2, remaining.clone())][1] >= new_score { //  
//                 continue;
//             }
//             visit_queue.push_back(((new_pos1, new_pos2, toa1, toa2, remaining.clone()), [new_t, new_score]));
//             visited.insert((new_pos1, new_pos2, remaining.clone()), [new_t, new_score]);
//         }
//     }
//     println!("{}", max);
//     *visited.values().map(|[_, s]| s).max().unwrap() as u64
// }

fn get_neighbours2(pos: usize, paths: &Array2<i32>) -> Vec<usize> {
    return paths.slice(s![pos, ..]).iter().enumerate().filter(|(_, &v)| v > 0).map(|(i, _)| i).collect_vec();
}

fn bfs2(start: usize, paths: &Array2<i32>, valves: &Vec<usize>, remaining_valves: &Vec<usize>) -> u64 {
    
    let remaining_valves = remove_chamber(start, remaining_valves);

    let node = ((start, start, remaining_valves.clone()), [0, 0]);
    let mut visit_queue = VecDeque::from_iter([node.clone()]);
    let mut visited = HashMap::new();
    visited.insert((start, start, remaining_valves.clone()), node.1); // add lastpos
    let mut max = 0;

    while !visit_queue.is_empty() {
        let ((pos1, pos2, new_remaining_valves), [timestep, score]) = visit_queue.pop_front().unwrap();
        let new_t = timestep+1;
        if new_t >= 25 {
            continue;
        }
        let mut neighbours1 = get_neighbours2(pos1, paths);
        let mut neighbours2 = get_neighbours2(pos2, paths);
        if new_remaining_valves.contains(&pos1) {
            neighbours1.push(pos1);
        }
        if new_remaining_valves.contains(&pos2) && pos1 != pos2 {
            neighbours2.push(pos2);
        }
        for new_pos1 in neighbours1 {
            for &new_pos2 in &neighbours2 {
                let mut remaining = new_remaining_valves.clone();
                let mut new_score = score;
                if new_pos1 == pos1 {
                    remaining = remove_chamber(pos1, &remaining);
                    new_score += valves[pos1] * (26-new_t);
                }
                if new_pos2 == pos2 {
                    remaining = remove_chamber(pos2, &remaining);
                    new_score += valves[pos2] * (26-new_t);
                }
                max = new_score.max(max);
                if visited.contains_key(&(new_pos1, new_pos2, remaining.clone())) && visited[&(new_pos1, new_pos2, remaining.clone())][1] >= new_score { //  
                    continue;
                }
                visit_queue.push_back(((new_pos1, new_pos2, remaining.clone()), [new_t, new_score]));
                visited.insert((new_pos1, new_pos2, remaining.clone()), [new_t, new_score]);
            }
        }
    }
    println!("{}", max);
    *visited.values().map(|[_, s]| s).max().unwrap() as u64
}
