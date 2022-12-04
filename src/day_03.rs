use std::collections::HashSet;

/// https://adventofcode.com/2022/day/3
pub fn solve(input: String) -> (i32, i32) {
    (part_1(&input), part_2(&input))
}

fn part_1(input: &str) -> i32 {
    input.lines().fold(0, |sum, line| {
        let (first, second) = line.split_at(line.len() / 2);
        let first_items = char_set(first);
        let duplicate = second
            .chars()
            .find(|c| first_items.contains(&c))
            .expect("Should have duplicate");
        sum + get_priority(duplicate)
    })
}

fn part_2(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    lines.chunks(3).fold(0, |sum, chunk| {
        let all = char_set(&chunk.join(""));
        let unique = all
            .iter()
            .find(|c| {
                char_set(chunk.get(0).unwrap()).contains(c)
                    && char_set(chunk.get(1).unwrap()).contains(c)
                    && char_set(chunk.get(2).unwrap()).contains(c)
            })
            .expect("Should have same char");
        sum + get_priority(*unique)
    })
}

fn char_set(str: &str) -> HashSet<char> {
    HashSet::from_iter(str.chars())
}

fn get_priority(char: char) -> i32 {
    char as i32
        - if char.is_lowercase() {
            97 - 1
        } else {
            65 - 26 - 1
        }
}
