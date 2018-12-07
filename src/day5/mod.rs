pub mod part1 {
    use super::*;

    pub fn run() -> String {
        let polymer = parse_input();
        react_polymer(&polymer).len().to_string()
    }
}

pub mod part2 {
    use super::*;
    pub fn run() -> String {
        let polymer = parse_input();
        let types = "abcdefghijklmnopqrstuvwxyz";

        let mut min_react_len = std::usize::MAX;
        for t in types.chars() {
            let type_removed = String::from_iter(polymer.chars().filter(|c| !same_type(t, *c)));
            let reacted_len = react_polymer(&type_removed).len();
            if reacted_len < min_react_len {
                min_react_len = reacted_len;
            }
        }

        min_react_len.to_string()
    }

    fn same_type(c1: char, c2: char) -> bool {
        c1.to_lowercase().next() == c2.to_lowercase().next()
    }
}

use std::iter::FromIterator;

fn react_polymer(polymer: &String) -> String {
    let mut result = Vec::new();

    for u1 in polymer.chars() {
        match result.pop() {
            None => result.push(u1),
            Some(u2) => if !reacts(u1, u2) {
                result.push(u2);
                result.push(u1);
            }
        }
    }
    String::from_iter(result)
}

fn reacts(u1: char, u2:char) -> bool {
    u1.is_lowercase() != u2.is_lowercase() && u1.to_lowercase().next() == u2.to_lowercase().next()
}

fn parse_input() -> String {
    let input = include_str!("input.txt");
    input.trim().to_string()
}
