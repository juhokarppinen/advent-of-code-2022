use regex::Regex;

/// https://adventofcode.com/2022/day/5
pub fn solve(input: String) -> (String, String) {
    let mut stacks: [Vec<char>; 9] = [
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];

    for line in input
        .lines()
        .take_while(|line| line.contains('['))
        .collect::<Vec<&str>>()
        .iter()
        .rev()
    {
        for i in 0..9 {
            let n = i * 4 + 1;
            let c = line.chars().nth(n).expect("Should be a character");
            if !c.is_whitespace() {
                stacks[i].push(c)
            }
        }
    }

    let mut stacks_2 = stacks.clone();

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").expect("Should be valid regexp");

    let parse_line = |line| {
        let caps = re.captures(line).unwrap();
        let caps: [usize; 3] = caps
            .iter()
            .skip(1)
            .map(|c| c.unwrap().as_str().parse::<usize>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        caps
    };

    for line in input.lines().skip_while(|line| !line.contains("move")) {
        let [n, from, to] = parse_line(line);

        // Part 1
        for _ in 0..n {
            let c = stacks[from - 1].pop();
            if let Some(c) = c {
                stacks[to - 1].push(c)
            }
        }

        // Part 2
        let fr = &stacks_2[from - 1];
        let start: String = fr.iter().take(fr.len() - n).collect();
        let mut new_stack: String = stacks_2[to - 1].iter().collect();
        let end: String = fr.iter().skip(start.len()).collect();
        new_stack.push_str(&end);
        stacks_2[from - 1] = start.chars().collect();
        stacks_2[to - 1] = new_stack.chars().collect();
    }

    let part_1 = stacks.iter().fold("".to_string(), |mut s, stack| {
        s.push(*stack.get(stack.len() - 1).unwrap());
        s
    });
    let part_2 = stacks_2.iter().fold("".to_string(), |mut s, stack| {
        s.push(*stack.get(stack.len() - 1).unwrap());
        s
    });

    (part_1, part_2)
}
