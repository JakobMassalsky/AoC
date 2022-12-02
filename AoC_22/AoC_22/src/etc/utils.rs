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
