/// https://adventofcode.com/2022/day/2
pub fn solve(input: String) -> (i32, i32) {
    (part_1(&input), part_2(&input))
}

fn part_1(input: &str) -> i32 {
    let mut score = 0;
    for line in input.lines() {
        let hands = parse_1(&line);
        let result = get_round_result(&hands);
        score += get_round_score(&result) + get_hand_score(&hands.1);
    }
    score
}

fn part_2(input: &str) -> i32 {
    let mut score = 0;
    for line in input.lines() {
        let (hand, result) = parse_2(&line);
        let my_hand = get_match_hand((&hand, &result));
        score += get_round_score(&result) + get_hand_score(&my_hand);
    }
    score
}

fn parse_1(line: &str) -> (Hand, Hand) {
    let hands: Vec<&str> = line.split(" ").collect();
    let a = match &hands.get(0) {
        Some(&"A") => Hand::Rock,
        Some(&"B") => Hand::Paper,
        Some(&"C") => Hand::Scissors,
        _ => panic!("This should not happen"),
    };
    let b = match &hands.get(1) {
        Some(&"X") => Hand::Rock,
        Some(&"Y") => Hand::Paper,
        Some(&"Z") => Hand::Scissors,
        _ => panic!("This should not happen"),
    };
    (a, b)
}

fn parse_2(line: &str) -> (Hand, RoundResult) {
    let hands: Vec<&str> = line.split(" ").collect();
    let a = match &hands.get(0) {
        Some(&"A") => Hand::Rock,
        Some(&"B") => Hand::Paper,
        Some(&"C") => Hand::Scissors,
        _ => panic!("This should not happen"),
    };
    let b = match &hands.get(1) {
        Some(&"X") => RoundResult::Lose,
        Some(&"Y") => RoundResult::Draw,
        Some(&"Z") => RoundResult::Win,
        _ => panic!("This should not happen"),
    };
    (a, b)
}

fn get_hand_score(hand: &Hand) -> i32 {
    match hand {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    }
}

fn get_round_result(hands: &(Hand, Hand)) -> RoundResult {
    if hands.0 == hands.1 {
        RoundResult::Draw
    } else {
        match hands {
            (Hand::Rock, Hand::Paper) => RoundResult::Win,
            (Hand::Paper, Hand::Scissors) => RoundResult::Win,
            (Hand::Scissors, Hand::Rock) => RoundResult::Win,
            _ => RoundResult::Lose,
        }
    }
}

fn get_round_score(result: &RoundResult) -> i32 {
    match result {
        RoundResult::Win => 6,
        RoundResult::Draw => 3,
        RoundResult::Lose => 0,
    }
}

fn get_match_hand(instruction: (&Hand, &RoundResult)) -> Hand {
    match instruction {
        (Hand::Rock, RoundResult::Win) => Hand::Paper,
        (Hand::Rock, RoundResult::Lose) => Hand::Scissors,
        (Hand::Paper, RoundResult::Win) => Hand::Scissors,
        (Hand::Paper, RoundResult::Lose) => Hand::Rock,
        (Hand::Scissors, RoundResult::Win) => Hand::Rock,
        (Hand::Scissors, RoundResult::Lose) => Hand::Paper,
        (_, RoundResult::Draw) => *instruction.0,
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

enum RoundResult {
    Win,
    Lose,
    Draw,
}
