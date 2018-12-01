pub mod part1 {
    use super::*;

    pub fn run() {
        println!("Running Day 1, Part 1");
        let result: i32 = parse_input().into_iter().sum();
        println!("Result: {:?}", result);
    }
}

pub mod part2 {
    use super::*;
    use std::collections::HashSet;
    pub fn run() {
        println!("Running Day 1, Part 2");
        let mut frequencies = parse_input().into_iter()
            .cycle()
            .scan(0, |f, c| {
                *f = *f + c;
                Some(*f)
            });

        let mut freq_seen = HashSet::new();
        let result = frequencies.find(|c| !freq_seen.insert(*c));
        match result {
            Some(r) => println!("Result: {:?}", r),
            None => println!("Result: Did not find a repeating frequency")
        }
    }
}

fn parse_input() -> Vec<i32> {
    let input = include_str!("input.txt");
    input.split_whitespace()
        .map(|s| s.parse::<i32>().expect("failed to parse number"))
        .collect()
}
