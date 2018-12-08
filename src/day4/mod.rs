pub mod part1 {
    use super::*;
    use std::collections::HashMap;

    pub fn run() -> String {
        let night_shifts = parse_input();

        //Find the guard with most total time asleep
        let mut guards_total_asleep = HashMap::new();
        for ns in &night_shifts {
            let total_asleep = guards_total_asleep.entry(ns.guard).or_insert(0);
            *total_asleep += ns.minutes_asleep();
        }
        let (most_asleep,_) = guards_total_asleep.iter().max_by(|a,b| a.1.cmp(b.1)).unwrap();

        //Find the minute he is the most asleep
        let mut minutes = [0; 60];
        for ns in night_shifts.iter().filter(|n| n.guard == *most_asleep) {
            for i in 0..59 {
                if ns.asleep[i] {
                    minutes[i] += 1;
                }
            }
        }
        let (minute, _) = minutes.iter().enumerate().max_by(|(_, v1), (_, v2)| v1.cmp(v2)).unwrap();

        (*most_asleep * (minute as i32)).to_string()
    }
}

pub mod part2 {
    use super::*;
    use std::collections::HashMap;

    pub fn run() -> String {
        let night_shifts = parse_input();

        //Compute guards asleep frequencies
        let mut guard_minutes_asleep: HashMap<i32, [i32;60]> = HashMap::new();
        for ns in &night_shifts {
            let minutes_asleep = guard_minutes_asleep.entry(ns.guard).or_insert([0;60]);
            for i in 0..59 {
                if ns.asleep[i] {
                    minutes_asleep[i] += 1;
                }
            }
        }

        //Find the guard with the most frequent minute
        let (most_asleep, minutes) = guard_minutes_asleep.iter().max_by(|(_, asleep1), (_, asleep2)| {
            asleep1.iter().max().cmp(&asleep2.iter().max())
        }).unwrap();

        let (minute, _) = minutes.iter().enumerate().max_by(|(_, v1), (_, v2)| v1.cmp(v2)).unwrap();

        (most_asleep * (minute as i32)).to_string()
    }
}

use regex::Regex;
use lazy_static::lazy_static;

fn parse_input() -> Vec<NightShift> {
    let input = include_str!("input.txt");
    let mut lines: Vec<&str> = input.lines().collect();
    lines.sort();


    let mut result = Vec::new();
    let mut itr = lines.into_iter().peekable();
    while let Some(sl) = itr.next() {
        match start_line(sl) {
            None => {},
            Some(guard_id) => {

                let mut asleep = [false; 60];
                while let Some(al) = itr.peek() {
                    match asleep_line(al) {
                        None => break,
                        Some(s) => {
                            let _ = itr.next();
                            let e = wakes_up_line(itr.next().unwrap()).expect("Failed to get matching wakeup");
                            for i in s..e {
                                asleep[i] = true
                            }
                        }
                    }
                }

                result.push(NightShift {
                    guard: guard_id,
                    asleep: asleep
                })
            }
        }
    }

    result
}

fn start_line(s: &str) -> Option<i32> {
    lazy_static! {
        static ref start_regex: Regex = Regex::new(r"\[.*\] Guard #(\d*) begins shift").unwrap();
    }
    start_regex.captures(s.trim()).map(|c| c[1].parse().unwrap())
}

fn asleep_line(s: &str) -> Option<usize> {
    lazy_static! {
        static ref asleep_regex: Regex = Regex::new(r"\[.* 00:(\d*)\] falls asleep").unwrap();
    }
    asleep_regex.captures(s).map(|c| c[1].parse().unwrap())
}

fn wakes_up_line(s: &str) -> Option<usize> {
    lazy_static! {
        static ref wakesup_regex: Regex = Regex::new(r"\[.* 00:(\d*)\] wakes up").unwrap();
    }
    wakesup_regex.captures(s).map(|c| c[1].parse().unwrap())
}

struct NightShift {
    guard: i32,
    asleep: [bool; 60]
}

impl NightShift {
    fn minutes_asleep(&self) -> i32 {
        self.asleep.into_iter().filter(|a| **a).count() as i32
    }
}
