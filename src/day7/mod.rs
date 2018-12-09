pub mod part1 {
    use super::*;

    pub fn run() -> String {
        let steps = parse_input();
        let mut completed_steps: Vec<char> = Vec::new();

        while let Some(mut to_complete) = find_can_be_completed(&steps, &completed_steps) {
            to_complete.sort();
            completed_steps.push(*to_complete.first().unwrap());
        }

        completed_steps.iter().collect()
    }

}

pub mod part2 {
    use super::*;
    pub fn run() -> String {
        //Pretty hacked together solution that does not give the right answer.  I think its close
        //to the right solution, but something is probably wrong with how 'time' is calculated.
        //Works on the example using 2 workers...
        let steps = parse_input();
        let mut completed_steps: Vec<char> = Vec::new();
        let mut workers = [None;5];
        let mut current_minute = 0;

        while let Some(mut to_complete) = find_can_be_completed(&steps, &completed_steps) {
            to_complete.sort();
            for tc in to_complete {
                //dont assign steps that are already being worked on
                if workers.iter().any(|w| match w {
                    None => false,
                    Some((s, _)) => *s == tc
                }) { break; }

                //assign step to worker
                for worker in &mut workers {
                    if worker.is_none() {
                        *worker = Some((tc, current_minute + step_time(tc)));
                        break;
                    }
                }
            }

            println!("{:?}", workers);
            //find worker that completes work first, and move time!
            let next_worker = workers.iter_mut().filter(|w| w.is_some()).min_by(|w1, w2| {
                w1.unwrap().1.cmp(&w2.unwrap().1)
            }).unwrap();
            current_minute = next_worker.unwrap().1;
            completed_steps.push(next_worker.unwrap().0);
            *next_worker = None;
        }

        current_minute.to_string()
    }

}

fn find_can_be_completed(steps: &HashMap<char, Vec<char>>, completed_steps: &Vec<char>) -> Option<Vec<char>> {
    let result: Vec<char> = steps.iter()
        .filter(|(step, befores)| !completed_steps.contains(step) && befores.iter().all(|b| completed_steps.contains(b)))
        .map(|(step, _)| step.clone())
        .collect();
    if result.len() > 0 { Some(result) } else { None }
}

static ALPHABET: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
fn step_time(s: char) -> i32 {
    61 + ALPHABET.iter().position(|a| *a == s.to_lowercase().next().unwrap()).unwrap() as i32
}

use regex::Regex;
use std::collections::hash_map::HashMap;

fn parse_input() -> HashMap<char, Vec<char>> {
    let input = include_str!("input.txt");
    let re = Regex::new(r"Step (\S) must be finished before step (\S) can begin.").unwrap();
    let mut result = HashMap::new();

    for l in input.lines() {
        let c = re.captures(l).unwrap();
        let before = c[1].parse().unwrap();
        let step = c[2].parse().unwrap();

        //ensure that steps with no depedent steps are added
        result.entry(before).or_insert(Vec::new());

        let befores = result.entry(step).or_insert(Vec::new());
        befores.push(before);
    }

    result
}
