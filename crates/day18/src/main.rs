use std::{
    collections::HashSet,
    fmt::{Display, Write},
};

use anyhow::Result;
use regex::Regex;
fn main() {
    println!("{}", i64::MAX);
    solve(include_str!("../input.txt")).unwrap();
}

fn solve(input: &str) -> Result<()> {
    let plan: Plan = parse(input)?;
    let dig = dig(&plan);
    println!("Part 1: {}", dig.dug.len());

    println!("Part 2: {}", part2(&plan));
    Ok(())
}

fn dig(plan: &Plan) -> Dig {
    let mut dig: Dig = Default::default();
    dig.dug.insert(Default::default());

    for step in plan.steps.iter() {
        for _ in 0..step.length {
            dig.digger = dig.digger.mv(step.dir);
            dig.dug.insert(dig.digger);
        }
    }

    dig.fill();

    dig
}

#[derive(Debug, Default)]
struct Dig {
    dug: HashSet<Pos>,
    digger: Pos,
}

impl Dig {
    fn fill(&mut self) {
        let (max_x, max_y) = self.dug.iter().fold((0, 0), |(max_x, max_y), pos| {
            (max_x.max(pos.x), max_y.max(pos.y))
        });

        let mut new_digs = HashSet::new();

        let (min_x, min_y) = self
            .dug
            .iter()
            .fold((i64::MAX, i64::MAX), |(max_x, max_y), pos| {
                (max_x.min(pos.x), max_y.min(pos.y))
            });

        for x in min_x..=max_x {
            let mut inside = false;
            for y in min_y..=max_y {
                if self.dug.contains(&Pos { x, y }) {
                    if self.dug.contains(&Pos { x: x - 1, y }) {
                        inside = !inside;
                    }
                } else if inside {
                    new_digs.insert(Pos { x, y });
                }
            }
        }

        self.dug.extend(new_digs);
    }
}

impl Display for Dig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (max_x, max_y) = self.dug.iter().fold((0, 0), |(max_x, max_y), pos| {
            (max_x.max(pos.x), max_y.max(pos.y))
        });

        let (min_x, min_y) = self
            .dug
            .iter()
            .fold((i64::MAX, i64::MAX), |(max_x, max_y), pos| {
                (max_x.min(pos.x), max_y.min(pos.y))
            });
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if self.dug.contains(&Pos { x, y }) {
                    f.write_char('#')?;
                } else {
                    f.write_char('.')?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse(input: &str) -> Result<Plan> {
    let re = Regex::new(r#"(?<dir>L|R|U|D) (?<len>\d+) \(#(?<color>[0-9a-f]{6})\)"#).unwrap();
    let steps = re
        .captures_iter(input)
        .map(|cap| {
            let dir = match &cap["dir"] {
                "L" => Ok(Dir::West),
                "R" => Ok(Dir::East),
                "U" => Ok(Dir::North),
                "D" => Ok(Dir::South),
                _ => Err(anyhow::anyhow!("can not happend")),
            }?;

            let length = cap["len"].parse::<i64>()?;
            let color = Color(cap["color"].to_string());
            Ok(Step { dir, length, color })
        })
        .collect::<Result<Vec<Step>>>()?;
    Ok(Plan { steps })
}

#[derive(Debug)]
struct Plan {
    steps: Vec<Step>,
}

#[derive(Debug)]
struct Step {
    dir: Dir,
    length: i64,
    color: Color,
}

impl Step {
    fn from_color(&self) -> Self {
        let dir = match self.color.0.chars().last().unwrap() {
            '0' => Dir::East,
            '1' => Dir::South,
            '2' => Dir::West,
            '3' => Dir::North,
            _ => panic!(),
        };

        let length = i64::from_str_radix(&self.color.0[0..5], 16).unwrap();
        Self {
            dir,
            length,
            color: self.color.clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct Color(String);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn mv(&self, dir: Dir) -> Pos {
        self.mv_at(dir, 1)
    }
    fn mv_at(&self, dir: Dir, length: i64) -> Pos {
        match dir {
            Dir::North => Pos {
                x: self.x - length,
                y: self.y,
            },
            Dir::South => Pos {
                x: self.x + length,
                y: self.y,
            },
            Dir::East => Pos {
                x: self.x,
                y: self.y + length,
            },
            Dir::West => Pos {
                x: self.x,
                y: self.y - length,
            },
        }
    }
}

fn part2(plan: &Plan) -> i64 {
    let steps = plan
        .steps
        .iter()
        .map(|s| s.from_color())
        .collect::<Vec<_>>();
    let mut pos: Pos = Default::default();
    let mut edges = Vec::with_capacity(steps.len());
    edges.push(pos);
    for step in steps {
        pos = pos.mv_at(step.dir, step.length);
        edges.push(pos);
    }

    picks(&edges)
}

fn shoelace(edges: &[Pos]) -> i64 {
    let edges_next = edges.iter().skip(1);
    let last = vec![(&edges[edges.len() - 1], &edges[0])];
    edges
        .iter()
        .zip(edges_next)
        .chain(last)
        .fold(0, |acc, (i, i_plus_one)| {
            acc + ((i.x + i_plus_one.x) * (i_plus_one.y - i.y)) / 2
        })
        .abs()
}

fn picks(edges: &[Pos]) -> i64 {
    let i = shoelace(edges);
    let b = edges
        .iter()
        .zip(edges.iter().skip(1))
        .map(|(a, b)| b.x.abs_diff(a.x) + b.y.abs_diff(a.y))
        .sum::<u64>() as i64;

    i + b / 2 + 1
}
