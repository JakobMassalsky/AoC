use itertools::{Itertools, Combinations};

use crate::{Solution, SolutionPair, etc::utils};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {

    const Y: i64 = 2000000;
    // const Y: i64 = 10;

    let lines = utils::read_lines("./input/input_15");
    let mut sensors = lines.iter().map(|s| utils::extract_ints(s)).collect_vec();
    for sensor in sensors.iter_mut() {
        let d = get_sensor_d(&sensor);
        sensor.push(d);
    }
    
    let mut intersections = vec![];

    for points in sensors.iter() {
        let (xs, ys) = (points[0], points[1]);
        let md = points[4];
        let (bottom, top, left, right) = (ys - md, ys + md, xs - md, xs + md); 
        if bottom > Y || top < Y { continue; }

        let dy = (Y - ys).abs();
        intersections.push([left + dy, 1]);
        intersections.push([right - dy, -1]);
    }

    intersections.sort_by(|[x1, _], [x2, _]| x1.cmp(x2));

    let mut sol1: i64 = 0;
    let mut depth = 0;
    let mut last_x = intersections[0][0];
    
    for [x, e] in intersections {
        depth += e;
        if depth == 0 {
            sol1 += x - last_x;
        }
        if depth == 1 && e == 1 {
            last_x = x;
        }
    }

    let mut sol2: i64 = 0;

    /*  Walk around the edges of each diamond and find the 
        first point that is not in range of another sensor */ 
    // const MAX: i64 = 4000000;
    // for points in sensors.iter() {
    //     let (xs, ys) = (points[0], points[1]);
    //     let md = points[4];
    //     let (left, right) = (xs - md, xs + md); 
    //     for x in (left-1).max(0)..=(right+1).min(MAX) {
    //         let d = (x - left + 1).min(- x + right + 1);
    //         sol2 = is_in_sensor_range([x, ys + d], &sensors, MAX);
    //         if sol2 > 0 { break; }
    //         sol2 = is_in_sensor_range([x, ys - d], &sensors, MAX);
    //         if sol2 > 0 { break; }
    //     }
    //     if sol2 > 0 { break; }
    // }

    let mut pairs = vec![];
    for (s1, s2) in sensors.iter().tuple_combinations() {
        let d = (s1[0] - s2[0]).abs() + (s1[1] - s2[1]).abs();
        if d == s1[4] + s2[4] + 2 {
            pairs.push([s1, s2]);
            if pairs.len() == 2 { 
                break; 
            }
        }
    }
    let s1 = (pairs[0][1][0] - pairs[0][0][0]).signum();
    let s2 = (pairs[1][1][0] - pairs[1][0][0]).signum();
    let x1 = pairs[0][0][0] + pairs[0][0][4]*s1 + s1;
    let y1 = pairs[0][0][1];
    let x2 = pairs[1][0][0] + pairs[1][0][4]*s2 + s2;
    let y2 = pairs[1][0][1];
    if s1 - (pairs[0][0][1] + pairs[0][1][1]).signum() == 0 {
        sol2 = get_point(x2, y2, x1, y1); // up left
    } else {
        sol2 = get_point(x1, y1, x2, y2); // up right
    }

    (Solution::I64(sol1), Solution::I64(sol2))
}

fn get_point(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    let x = (x1 + x2 - y1 + y2) / 2;
    let y = (-x1 + x2 + y1 + y2) / 2;
    x * 4000000 + y
}

fn is_in_sensor_range(p: [i64; 2], sensors: &Vec<Vec<i64>>, max: i64) -> i64 {
    if p[1] < 0 || p[1] > max { 
        return 0;
    }
    if sensors.iter().any(|s| (p[0] - s[0]).abs() + (p[1] - s[1]).abs() <= s[4]) {
        return 0;
    }
    p[0] * 4000000 + p[1]
}

fn get_sensor_d(points: &Vec<i64>) -> i64 {
    let (xs, ys, xb, yb) = (points[0], points[1], points[2], points[3]);
    (xs - xb).abs() + (ys - yb).abs()
}
