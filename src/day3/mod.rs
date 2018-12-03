use regex::Regex;

pub mod part1 {
    use super::*;

    pub fn run() -> String {
        let claims = parse_input();
        let mut fabrics = [[0; 1000]; 1000];

        for c in &claims {
            for x in c.from_left..c.x + c.from_left {
                let x = x as usize;
                for y in c.from_top..c.y + c.from_top {
                    let y = y as usize;
                    fabrics[x][y] = fabrics[x][y] + 1
                }
            }
        }

        let claimed: i32 = fabrics
            .into_iter()
            .map(|r| r.into_iter().filter(|r| **r > 1).count() as i32)
            .sum();
        claimed.to_string()
    }
}

pub mod part2 {
    use super::*;

    pub fn run() -> String {
        let claims = parse_input();
        let mut fabrics = vec![vec![vec![]; 1000]; 1000];

        for c in &claims {
            for x in c.from_left..c.x + c.from_left {
                let x = x as usize;
                for y in c.from_top..c.y + c.from_top {
                    let y = y as usize;
                    fabrics[x][y].push(c.id);
                }
            }
        }
        let no_overlap = claims.into_iter().find(|c| {
            for x in c.from_left..c.x + c.from_left {
                let x = x as usize;
                for y in c.from_top..c.y + c.from_top {
                    let y = y as usize;
                    if fabrics[x][y].len() > 1 {
                        return false;
                    }
                }
            }
            true
        });
        format!("{:?}", no_overlap)
    }
}

#[derive(Debug)]
struct Claim {
    id: i32,
    from_left: i32,
    from_top: i32,
    x: i32,
    y: i32,
}

fn parse_input() -> Vec<Claim> {
    let input = include_str!("input.txt");
    let re = Regex::new(r"#(\d*) @ (\d*),(\d*): (\d*)x(\d*)").unwrap();
    re.captures_iter(input)
        .map(|c| Claim {
            id: c[1].parse().unwrap(),
            from_left: c[2].parse().unwrap(),
            from_top: c[3].parse().unwrap(),
            x: c[4].parse().unwrap(),
            y: c[5].parse().unwrap(),
        })
        .collect()
}
