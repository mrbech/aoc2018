pub mod part1 {
    use super::*;

    pub fn run() -> String {
        parse_input().into_iter().sum::<i32>().to_string()
    }
}

pub mod part2 {
    use super::*;
    use std::collections::HashSet;
    pub fn run() -> String {
        let mut frequencies = parse_input().into_iter()
            .cycle()
            .scan(0, |f, c| {
                *f = *f + c;
                Some(*f)
            });

        let mut freq_seen = HashSet::new();
        let result = frequencies.find(|c| !freq_seen.insert(*c));
        match result {
            Some(r) => r.to_string(),
            None => "Did not find a repeating frequency".to_string()
        }
    }
}

fn parse_input() -> Vec<i32> {
    let input = include_str!("input.txt");
    input.split_whitespace()
        .map(|s| s.parse::<i32>().expect("failed to parse number"))
        .collect()
}
