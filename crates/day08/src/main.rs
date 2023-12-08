use std::collections::HashMap;

use regex::Regex;

fn main() {
    solve_one(include_str!("../input.txt")).unwrap();
    solve_two(include_str!("../input.txt")).unwrap();
}

fn solve_one(input: &str) -> Result<(), anyhow::Error> {
    let (rules, graph) = parse(input)?;

    println!("Rules: {}", rules.len());
    let mut current = "AAA";
    let mut steps = 0;
    let mut rules_prog = rules.chars().cycle();
    while current != "ZZZ" {
        current = if rules_prog.next().unwrap() == 'L' {
            &graph.get(current).unwrap().left
        } else {
            &graph.get(current).unwrap().right
        };
        steps += 1;
    }

    println!("Part 1: {}", steps);

    Ok(())
}

fn solve_two(input: &str) -> Result<(), anyhow::Error> {
    let (rules, graph) = parse(input)?;
    let mut points = graph
        .keys()
        .filter(|k| k.ends_with('A'))
        .collect::<Vec<_>>();

    let periods = points
        .iter()
        .map(|start| {
            let mut current = *start;
            let mut steps = 0;
            let mut rules_prog = rules.chars().cycle();
            while !current.ends_with('Z') {
                current = if rules_prog.next().unwrap() == 'L' {
                    &graph.get(current).unwrap().left
                } else {
                    &graph.get(current).unwrap().right
                };
                steps += 1;
            }
            steps
        })
        .collect::<Vec<_>>();

    println!("periods: {:?}", periods);
    println!("Part 2: {}", lcm(&periods));

    Ok(())
}

fn solve_two_naive(input: &str) -> Result<(), anyhow::Error> {
    let (rules, graph) = parse(input)?;
    let mut points = graph
        .keys()
        .filter(|k| k.ends_with('A'))
        .collect::<Vec<_>>();
    let mut steps = 0;
    let mut rules_prog = rules.chars().cycle();
    while !points.iter().all(|p| p.ends_with('Z')) {
        points = points
            .into_iter()
            .map(|current| {
                let new = if rules_prog.next().unwrap() == 'L' {
                    &graph.get(current).unwrap().left
                } else {
                    &graph.get(current).unwrap().right
                };
                new
            })
            .collect();
        steps += 1;
    }
    println!("Steps: {}", steps);
    Ok(())
}

fn parse(input: &str) -> Result<(String, HashMap<String, Node>), anyhow::Error> {
    let (rule, graph_nodes) = input.split_once("\n\n").unwrap();
    let re = Regex::new(r#"(?P<name>.{3}) = \((?P<left>.{3}), (?P<right>.{3})\)"#).unwrap();
    let graph = graph_nodes
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            (
                caps["name"].to_string(),
                Node {
                    left: caps["left"].to_string(),
                    right: caps["right"].to_string(),
                },
            )
        })
        .collect::<HashMap<String, Node>>();
    Ok((rule.to_string(), graph))
}

struct Node {
    left: String,
    right: String,
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
