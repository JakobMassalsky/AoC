mod days;
mod etc;

use etc::solution::Solution;
use days::{day_01, day_02, day_03, day_04, day_05,
           day_06, day_07, day_08, day_09, day_10,
           day_11, day_12, day_13, day_14, day_15,
           day_16, day_17, day_18, day_19, day_20,
           day_21, day_22, day_23, day_24, day_25};
use std::env;
use std::time::Instant;

pub type SolutionPair = (Solution, Solution);

fn main() {
    let args: Vec<String> = env::args().collect();
    let days: Vec<u8>;
    if args.len() < 2 {
        days = vec![4];
    } else {
        days = args[1..].iter()
        .map(|x| x.parse().unwrap_or_else(|v| panic!("Not a valid day: {}", v)))
        .collect();
    }

    let mut runtime = 0.0;

    for day in days {
        let func = get_day_solver(day);

        let time = Instant::now();
        let (p1, p2) = func();
        let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
        
        println!("\n=== Day {:02} ===", day);
        println!("  · Part 1: {}", p1);
        println!("  · Part 2: {}", p2);
        println!("  · Elapsed: {:.4} ms", elapsed_ms);

        runtime += elapsed_ms;
    }

    println!("Total runtime: {:.4} ms", runtime);
}

fn get_day_solver(day: u8) -> fn() -> SolutionPair {
    match day {
         1 => day_01::solve,
         2 => day_02::solve,
         3 => day_03::solve,
         4 => day_04::solve,
         5 => day_05::solve,
         6 => day_06::solve,
         7 => day_07::solve,
         8 => day_08::solve,
         9 => day_09::solve,
        10 => day_10::solve,
        11 => day_11::solve,
        12 => day_12::solve,
        13 => day_13::solve,
        14 => day_14::solve,
        15 => day_15::solve,
        16 => day_16::solve,
        17 => day_17::solve,
        18 => day_18::solve,
        19 => day_19::solve,
        20 => day_20::solve,
        21 => day_21::solve,
        22 => day_22::solve,
        23 => day_23::solve,
        24 => day_24::solve,
        25 => day_25::solve,
         _ => unimplemented!(),
    }
}

