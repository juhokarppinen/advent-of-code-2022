use std::fs;
use std::io;

const DAY: u8 = 0;

pub fn get_solution() -> io::Result<(u8, i32, i32)> {
    let input = fs::read_to_string(format!("./input/{DAY:0>2}.txt"))?;
    let (part_1, part_2) = solve(&input);
    Ok((DAY, part_1, part_2))
}

/// https://adventofcode.com/2022/day/0
fn solve(input: &str) -> (i32, i32) {
    (0, 0)
}
