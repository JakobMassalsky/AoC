use itertools::Itertools;

use crate::{Solution, SolutionPair, etc::utils};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {

    const Y: i64 = 2000000;
    // const Y: i64 = 10;
    const MAX: i64 = 4000000;

    let lines = utils::read_lines("./input/input_15");
    let sensors = lines.iter().map(|s| extract_ints(s)).collect_vec();
    let sensor_dmax = sensors.iter().map(get_sensor_d).collect_vec();

    let mut intersections = vec![];

    for (i, points) in sensors.iter().enumerate() {
        let (xs, ys) = (points[0], points[1]);
        let md = sensor_dmax[i];
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

    for (i, points) in sensors.iter().enumerate() {
        let (xs, ys) = (points[0], points[1]);
        let md = sensor_dmax[i];
        let (left, right) = (xs - md, xs + md); 
        for x in (left-1).max(0)..=(right+1).min(MAX) {
            let d = (x - left + 1).min(- x + right + 1);
            sol2 = is_in_sensor_range([x, ys + d], &sensors, &sensor_dmax, MAX);
            if sol2 > 0 { break; }
            sol2 = is_in_sensor_range([x, ys - d], &sensors, &sensor_dmax, MAX);
            if sol2 > 0 { break; }
        }
        if sol2 > 0 { break; }
    }

    (Solution::I64(sol1), Solution::I64(sol2))
}

fn is_in_sensor_range(p: [i64; 2], sensors: &Vec<Vec<i64>>, dmax: &Vec<i64>, max: i64) -> i64 {
    if p[1] < 0 || p[1] > max { 
        return 0;
    }
    if sensors.iter().zip_eq(dmax.iter()).any(|(s, &d)| (p[0] - s[0]).abs() + (p[1] - s[1]).abs() <= d) {
        return 0;
    }
    p[0] * 4000000 + p[1]
}

fn get_sensor_d(points: &Vec<i64>) -> i64 {
    let (xs, ys, xb, yb) = (points[0], points[1], points[2], points[3]);
    (xs - xb).abs() + (ys - yb).abs()
}

fn extract_ints(line: &String) -> Vec<i64> {
    let mut num_buff = vec![];
    let mut nums = vec![];
    for c in line.chars() {
        if c.is_numeric() || c == '-' {
            num_buff.push(c);
        } else if !num_buff.is_empty() {
            nums.push(num_buff.iter().collect::<String>().parse::<i64>().unwrap());
            num_buff.clear();
        }
    }
    if !num_buff.is_empty() {
        nums.push(num_buff.iter().collect::<String>().parse::<i64>().unwrap());
    }
    nums
}
