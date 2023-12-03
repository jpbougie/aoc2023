use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use regex::Regex;

fn main() {
    solve(include_str!("../input.txt")).unwrap();
}

#[derive(Debug)]
struct Schematic {
    symbols: HashMap<(i32, i32), char>,
    numbers: Vec<(String, i32, i32)>,
}

impl FromStr for Schematic {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers_re = Regex::new(r#"\d+"#)?;
        let symbols_re = Regex::new(r#"[^\d.]"#)?;
        let mut result = Self {
            symbols: HashMap::new(),
            numbers: Vec::new(),
        };
        for (x, line) in s.lines().enumerate() {
            for m in numbers_re.find_iter(line) {
                result
                    .numbers
                    .push((m.as_str().to_string(), x as i32, m.start() as i32));
            }

            for m in symbols_re.find_iter(line) {
                result.symbols.insert(
                    (x as i32, m.start() as i32),
                    m.as_str().chars().next().unwrap(),
                );
            }
        }

        Ok(result)
    }
}

impl Schematic {
    fn part1(&self) -> u32 {
        self.numbers
            .iter()
            .filter_map(|(n, x, y)| {
                let check = (0..(n.len())).any(|i| {
                    for xx in -1..=1 {
                        for yy in -1..=1 {
                            if self.symbols.contains_key(&(x + xx, y + (i as i32) + yy)) {
                                return true;
                            }
                        }
                    }

                    false
                });
                if check {
                    Some(n.parse::<u32>().unwrap())
                } else {
                    None
                }
            })
            .sum()
    }

    fn part2(&self) -> u32 {
        self.symbols
            .iter()
            .filter_map(|((x, y), ch)| {
                if *ch != '*' {
                    return None;
                }

                let mut hit_zone = HashSet::new();
                for xx in -1..=1 {
                    for yy in -1..=1 {
                        hit_zone.insert((*x + xx, *y + yy));
                    }
                }

                let matches = self
                    .numbers
                    .iter()
                    .filter(|(n, x, y)| {
                        for pos in 0..(n.len() as i32) {
                            if hit_zone.contains(&(*x, *y + pos)) {
                                return true;
                            }
                        }

                        false
                    })
                    .collect::<Vec<_>>();

                if matches.len() != 2 {
                    return None;
                }

                Some(matches[0].0.parse::<u32>().unwrap() * matches[1].0.parse::<u32>().unwrap())
            })
            .sum()
    }
}

fn solve(input: &str) -> Result<(), anyhow::Error> {
    let schematic: Schematic = input.parse()?;
    println!("Part 1: {}", schematic.part1());
    println!("Part 2: {}", schematic.part2());

    Ok(())
}
