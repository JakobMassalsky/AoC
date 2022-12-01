use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input/input_01") {

        let mut sums: Vec<i32> = Vec::new();
        let mut sum = 0;
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(number) = line {
                if number.is_empty() {
                    sums.push(sum);
                    sum = 0;
                } else {
                    sum += number.parse::<i32>().unwrap();
                }
            }
        }
        sums.sort_by(|a, b| b.cmp(a));
        let cal: i32 = sums[0..3].iter().sum();
        println!("{}", cal);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
