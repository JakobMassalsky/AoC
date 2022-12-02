use crate::{Solution, SolutionPair, etc::utils};
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let lines = utils::read_lines("./input/input_02");
    let mut  s1 = 0;
    let mut  s2 = 0;
    for (e, m) in lines.iter().map(|x| x.split_once(" ").unwrap()) {
        s1 += match m { "X" => 1, "Y" => 2, _ => 3};
        s2 += match (m, e) {
            ("X", "B") | ("Y", "C") | ("Z", "A") => 0,
            ("X", "A") | ("Y", "B") | ("Z", "C") => 3,
            _ => 6,
        };

        s2 += match m { "X" => 0, "Y" => 3, _ => 6};
        s2 += match (m, e) {
            ("X", "B") | ("Y", "A") | ("Z", "C") => 1,
            ("X", "C") | ("Y", "B") | ("Z", "A") => 2,
            _ => 3,
        };
    }

    (Solution::U64(s1), Solution::U64(s2))
}
