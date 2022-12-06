/// https://adventofcode.com/2022/day/1
pub fn solve(input: String) -> (String, String) {
    let mut cals_per_elf: Vec<i32> = input
        .split("\n\n")
        .map(|c| c.lines().map(|line| line.parse().unwrap_or(0)).sum())
        .collect();
    cals_per_elf.sort_unstable();
    cals_per_elf.reverse();
    (
        cals_per_elf.iter().take(1).sum::<i32>().to_string(),
        cals_per_elf.iter().take(3).sum::<i32>().to_string(),
    )
}
