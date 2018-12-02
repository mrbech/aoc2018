pub mod part1 {
    use super::*;
    use std::collections::HashMap;

    fn char_occurences(s: &String) -> HashMap<char, i32> {
        let mut o = HashMap::new();
        for c in s.chars() {
            let count = o.entry(c).or_insert(0);
            *count += 1
        }
        o
    }

    pub fn run() -> String {
        let occurences = parse_input()
            .into_iter()
            .map(|s| char_occurences(&s))
            .fold((0,0), |a, o| {
                let twos = if o.values().any(|c| *c == 2) { 1 } else { 0 };
                let threes = if o.values().any(|c| *c == 3) { 1 } else { 0 };
                (a.0 + twos, a.1 + threes)
            });
        (occurences.0 * occurences.1).to_string()
    }
}

pub mod part2 {
    use super::*;
    use std::iter::FromIterator;

    fn find_similar_string(s1: &String, s2: &String) -> String {
        let chars = s1.chars().zip(s2.chars())
            .filter(|(a,b)| a == b)
            .map(|(a,_)| a);
        String::from_iter(chars)
    }

    pub fn run() -> String {
        let input = parse_input();
        for s1 in &input {
            for s2 in &input {
                let similar = find_similar_string(&s1, &s2);
                if s1.len() - similar.len() == 1 {
                    return similar
                }
            }
        }
        "Found no similar box ids".to_string()
    }
}

fn parse_input() -> Vec<String> {
    let input = include_str!("input.txt");
    input.split_whitespace().map(|s| s.to_string()).collect()
}
