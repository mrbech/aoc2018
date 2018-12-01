pub mod part1 {
    pub fn run() {
        println!("Running Day 1, Part 1");

        let input = include_str!("part1.txt");
        let result: i32 = input.split_whitespace()
            .map(|s| {
                let v: i32 = s[1..].parse().expect("failed to parse number");
                match s.chars().next().expect("failed to first char") {
                    '+' => v,
                    '-' => -v,
                    o => panic!("{} should have been an operator", o)
                }
            }).sum();

        println!("Result: {:?}", result);
    }
}
