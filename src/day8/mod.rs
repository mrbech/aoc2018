pub mod part1 {
    use super::*;

    pub fn run() -> String {
        let root = parse_input();
        sum_meta_data(&root).to_string()
    }

    fn sum_meta_data(n: &Node) -> i32 {
        n.meta_data.iter().sum::<i32>() + n.children.iter().map(|c| sum_meta_data(c)).sum::<i32>()
    }
}

pub mod part2 {
    use super::*;
    pub fn run() -> String {
        let root = parse_input();
        node_value(&root).to_string()
    }

    fn node_value(n: &Node) -> i32 {
        if n.children.len() == 0 {
            n.meta_data.iter().sum::<i32>()
        } else {
            n.meta_data.iter().map(|ci| {
                match n.children.get((*ci - 1) as usize) {
                    None => 0,
                    Some(c) => node_value(c)
                }
            }).sum::<i32>()
        }
    }
}

#[derive(Debug)]
struct Node {
    meta_data: Vec<i32>,
    children: Vec<Node>
}

fn parse_input() -> Node {
    let input = include_str!("input.txt");
    let mut numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("failed to parse number"))
        .collect();
    numbers.reverse();

    parse_node(&mut numbers)
}

fn parse_node(stack: &mut Vec<i32>) -> Node {
    let c_count = stack.pop().unwrap();
    let m_count = stack.pop().unwrap();

    let mut children = Vec::new();
    let mut meta_data = Vec::new();

    for _ in 0..c_count {
        children.push(parse_node(stack));
    }

    for _ in 0..m_count {
        meta_data.push(stack.pop().expect("something wrong with meta data count"));
    }

    Node {
        meta_data,
        children
    }
}
