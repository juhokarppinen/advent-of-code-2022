use std::fs;
use std::io;

const DAY: u8 = 1;

pub fn get_solution() -> io::Result<(u8, i32, i32)> {
    let input = fs::read_to_string(format!("./input/{DAY:0>2}.txt"))?;
    let (part_1, part_2) = solve(&input);
    Ok((DAY, part_1, part_2))
}

/// https://adventofcode.com/2022/day/1
fn solve(input: &str) -> (i32, i32) {
    let mut cals_per_elf: Vec<i32> = input
        .split("\n\n")
        .map(|c| c.lines().map(|line| line.parse().unwrap_or(0)).sum())
        .collect();
    cals_per_elf.sort_unstable();
    cals_per_elf.reverse();
    (
        cals_per_elf.iter().take(1).sum(),
        cals_per_elf.iter().take(3).sum(),
    )
}
