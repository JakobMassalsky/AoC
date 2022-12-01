use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input/input_01");
    let mut sums: Vec<i32> = Vec::new();
    let mut sum = 0;
    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        if line.is_empty() {
            sums.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }
    sums.sort_by(|a, b| b.cmp(a));
    let cal: i32 = sums[0..3].iter().sum();
    println!("{}", cal);
}

fn read_lines<P>(filename: P) -> Vec<String>
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
