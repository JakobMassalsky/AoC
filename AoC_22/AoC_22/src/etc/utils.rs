use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs::read_to_string;

pub fn read_lines<P>(filename: P) -> Vec<String>
where P: AsRef<Path> {
    let mut l: Vec<String> = Vec::new();
    if let Ok(file) = File::open(filename) {
        for line in io::BufReader::new(file).lines() {
            if let Ok(s) = line {
                l.push(s);
            }
        }
    }
    return l;
}

pub fn extract_ints(line: &String) -> Vec<i64> {
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
