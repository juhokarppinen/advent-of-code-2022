mod day_01;
mod day_02;
mod day_03;

/// https://adventofcode.com/2022
fn main() {
    let solvers = [day_01::solve, day_02::solve, day_03::solve];

    for (i, solve) in solvers.iter().enumerate() {
        let day = i + 1;
        println!(
            "Day {day:0>2}: {:?}",
            get_input(day).and_then(|input| Ok(solve(input)))
        );
    }
}

fn get_input(day: usize) -> std::io::Result<String> {
    std::fs::read_to_string(format!("./input/{day:0>2}.txt"))
}
