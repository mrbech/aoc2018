mod day1;
mod day2;
mod day3;
mod day5;

fn no_puzzle() -> String {
    panic!("puzzle not implemented")
}

fn main() {
    let puzzles = [
        [day1::part1::run, day1::part2::run],
        [day2::part1::run, day2::part2::run],
        [day3::part1::run, day3::part2::run],
        [no_puzzle, no_puzzle],
        [day5::part1::run, day5::part2::run],
    ];

    let args: Vec<String> = std::env::args().collect();
    let day: usize = args
        .get(1)
        .expect("Please provide a day")
        .parse()
        .expect("Day should be an unsigned int");
    let part: usize = args
        .get(2)
        .expect("Please provide a part")
        .parse()
        .expect("Part should be an unsigned int");

    let puzzle = puzzles
        .get(day - 1)
        .and_then(|d| d.get(part - 1))
        .expect("Could not find puzzle");

    println!("Running day {}, part {}", day, part);
    let result = puzzle();
    println!("Result: {}", result);
}
