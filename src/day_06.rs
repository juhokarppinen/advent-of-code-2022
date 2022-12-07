use std::collections::HashSet;

/// https://adventofcode.com/2022/day/6
pub fn solve(input: String) -> (String, String) {
    (
        (find_marker_position(&input, 4)).to_string(),
        (find_marker_position(&input, 14)).to_string(),
    )
}

fn find_marker_position(input: &str, length: usize) -> usize {
    &input
        .as_bytes()
        .windows(length)
        .position(|item| item.iter().collect::<HashSet<_>>().len() == length)
        .unwrap()
        + length
}
