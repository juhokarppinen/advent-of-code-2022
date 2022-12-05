/// https://adventofcode.com/2022/day/4
pub fn solve(input: String) -> (i32, i32) {
    input.lines().fold((0, 0), |(part_1, part_2), line| {
        let [(a, b), (p, q)]: [_; 2] = line
            .split(",")
            .map(parse_range)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        (
            part_1 + ((a >= p && b <= q) || (p >= a && q <= b)) as i32,
            part_2 + ((q >= a) && (p <= b)) as i32,
        )
    })
}

fn parse_range(range: &str) -> (i32, i32) {
    let (start, end) = range
        .split_once('-')
        .expect("Range should have a start and an end");
    (
        start.parse::<i32>().expect("Start should be an integer"),
        end.parse::<i32>().expect("End should be an integer"),
    )
}
