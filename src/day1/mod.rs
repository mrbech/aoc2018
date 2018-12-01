pub mod part1 {
    pub fn run() {
        println!("Running Day 1, Part 1");

        let input = include_str!("part1.txt");
        let result: i32 = input.split_whitespace()
            .map(|s| s.parse::<i32>().expect("failed to parse number"))
            .sum();

        println!("Result: {:?}", result);
    }
}
