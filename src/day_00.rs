use std::fs;
use std::io;

const DAY: &'static str = "01";

/// https://adventofcode.com/2022/day/0
pub fn get_solution() -> io::Result<(String, i32, i32)> {
    let input = fs::read_to_string(format!("./input/{DAY}.txt"))?;

    let part_1 = 0;
    let part_2 = 0;

    Ok((format!("Day {DAY}"), part_1, part_2))
}
