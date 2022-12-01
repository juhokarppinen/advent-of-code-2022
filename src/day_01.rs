use std::fs;
use std::io;

const DAY: &'static str = "01";

/// https://adventofcode.com/2022/day/1
pub fn get_solution() -> io::Result<(String, i32, i32)> {
    let input = fs::read_to_string(format!("./input/{DAY}.txt"))?;

    let mut calories_per_elf: Vec<i32> = input
        .split("\n\n")
        .map(|c| c.lines().map(|line| line.parse().unwrap_or(0)).sum())
        .collect();
    calories_per_elf.sort_unstable();
    calories_per_elf.reverse();

    let part_1 = sum(1, &calories_per_elf);
    let part_2 = sum(3, &calories_per_elf);

    Ok((format!("Day {DAY}"), part_1, part_2))
}

fn sum(n: usize, sums: &Vec<i32>) -> i32 {
    sums.iter().take(n).sum()
}
