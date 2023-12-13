use std::{collections::HashMap, num::ParseIntError, str::FromStr};

use anyhow::Result;
fn main() {
    solve(include_str!("../input.txt")).unwrap();
}

fn solve(input: &str) -> Result<()> {
    let lines = parse(input)?;
    let part1 = solve_part1_very_dumb(&lines);
    println!("Part 1: {}", part1);
    let part2 = solve_part2(&lines, 5);
    println!("Part 2: {}", part2);
    Ok(())
}

fn parse(input: &str) -> Result<Vec<Line>> {
    input.lines().map(|l| l.parse()).collect()
}

fn solve_part1_very_dumb(lines: &[Line]) -> usize {
    lines.iter().map(|l| solve_line_very_dumb(l)).sum()
}

fn solve_line_very_dumb(line: &Line) -> usize {
    let unknowns = line.count_unknowns();
    variants(unknowns, line.missing_damaged())
        .filter(|l| line.with_replacements_for_unknown(l).valid())
        .count()
}

fn solve_line(line: &Line, cache: &mut HashMap<Line, usize>) -> usize {
    if let Some(prev) = cache.get(line) {
        return *prev;
    }

    let unknowns = line.count_unknowns();
    if unknowns == 0 {
        if line.valid() {
            return 1;
        } else {
            return 0;
        }
    }

    let mut res = 0;
    let with_damaged = line.with_replacements_for_unknown(&[Cell::Damaged]);
    if with_damaged.valid_prefix() {
        let stripped = with_damaged.remove_prefix();
        res += solve_line(&stripped, cache);
    } else {
        //println!("{:?} is not a valid prefix", with_damaged);
    }

    let with_operational = line.with_replacements_for_unknown(&[Cell::Operational]);
    if with_operational.valid_prefix() {
        let stripped = with_operational.remove_prefix();
        res += solve_line(&stripped, cache);
    } else {
        //println!("{:?} is not a valid prefix", with_operational);
    }

    cache.insert(line.clone(), res);
    res
}

#[allow(dead_code)]
fn solve_part2_very_dumb(lines: &[Line]) -> usize {
    lines
        .iter()
        .map(|line| {
            let line = line.multiplied(5);
            let unknowns = line.count_unknowns();
            variants(unknowns, line.missing_damaged())
                .filter(|l| line.with_replacements_for_unknown(l).valid())
                .count()
        })
        .sum()
}

fn solve_part2(lines: &[Line], factor: usize) -> usize {
    let lines: Vec<Line> = lines.iter().map(|l| l.multiplied(factor)).collect();

    lines
        .iter()
        .map(|line| {
            let potentials = line.potentials();
            let res = potentials
                .iter()
                .map(|lines| {
                    lines
                        .iter()
                        .map(|l| {
                            let mut cache = HashMap::new();
                            solve_line(l, &mut cache)
                        })
                        .product::<usize>()
                })
                .sum::<usize>();

            dbg!(res)
        })
        .sum()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Line {
    grid: Vec<Cell>,
    groups: Vec<usize>,
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let (map, groups) = s
            .split_once(' ')
            .ok_or(anyhow::anyhow!("not a single space"))?;

        let grid = map
            .chars()
            .map(|ch| match ch {
                '#' => Ok(Cell::Damaged),
                '.' => Ok(Cell::Operational),
                '?' => Ok(Cell::Unknown),
                _ => Err(anyhow::anyhow!("Unknown char {}", ch)),
            })
            .collect::<Result<Vec<Cell>, _>>()?;

        let groups = groups
            .split(',')
            .map(|num| num.parse::<usize>())
            .collect::<Result<Vec<usize>, ParseIntError>>()?;

        Ok(Self { grid, groups })
    }
}

impl Line {
    fn remove_prefix(&self) -> Self {
        let mut new_grid = Vec::with_capacity(self.grid.len());
        let mut new_groups = Vec::with_capacity(self.groups.len());

        let mut groups_iter = self.groups.iter();
        let mut grid_iter = self.grid.iter();
        let mut cur_group = None;
        let mut prev = None;
        for cell in grid_iter.by_ref() {
            match cell {
                Cell::Operational => {
                    prev = Some(Cell::Operational);
                }
                Cell::Damaged => {
                    prev = Some(Cell::Damaged);
                    if cur_group.is_none() {
                        cur_group = groups_iter.next().copied();
                    }

                    if let Some(ref mut grp) = cur_group {
                        *grp -= 1;
                        if *grp == 0 {
                            cur_group = None;
                        }
                    }
                }
                Cell::Unknown => {
                    new_grid.push(Cell::Unknown);
                    break;
                }
            };
        }

        if let Some(mut cur_group) = cur_group {
            if prev == Some(Cell::Damaged) {
                cur_group += 1;
            }
            new_groups.push(cur_group);
        } else if prev == Some(Cell::Damaged) {
            new_groups.push(1);
        }

        for left in groups_iter {
            new_groups.push(*left);
        }

        if prev == Some(Cell::Damaged) {
            new_grid.insert(0, Cell::Damaged);
        }

        for left in grid_iter {
            new_grid.push(*left);
        }

        Self {
            grid: new_grid,
            groups: new_groups,
        }
    }

    fn multiplied(&self, factor: usize) -> Self {
        let mut new_grid = self.grid.clone();
        for _ in 1..factor {
            new_grid.push(Cell::Unknown);
            new_grid.append(&mut self.grid.clone());
        }
        Self {
            grid: new_grid,
            groups: self.groups.repeat(factor),
        }
    }

    fn subgroups(&self) -> Vec<Vec<Cell>> {
        self.grid
            .split(|cell| *cell == Cell::Operational)
            .filter_map(|spl| {
                if spl.is_empty() {
                    None
                } else {
                    Some(spl.to_vec())
                }
            })
            .collect()
    }

    fn potentials(&self) -> Vec<Vec<Line>> {
        let subgroups = self.subgroups();

        arrangements(Vec::new(), &subgroups, &self.groups)
    }

    fn valid_prefix(&self) -> bool {
        let mut groups = self.groups.iter();
        let mut current_group = 0;

        for cell in self.grid.iter() {
            match cell {
                Cell::Unknown => {
                    break;
                }
                Cell::Operational => {
                    if current_group == 0 {
                        continue;
                    }

                    let Some(expected) = groups.next() else {
                        return false;
                    };

                    if *expected != current_group {
                        return false;
                    }

                    current_group = 0;
                }
                Cell::Damaged => {
                    current_group += 1;
                }
            }
        }
        if current_group == 0 {
            return true;
        }
        let Some(expected) = groups.next() else {
            return false;
        };

        current_group <= *expected
    }

    fn valid(&self) -> bool {
        let mut computed_groups = Vec::new();
        let mut cur_group = 0;
        for cell in self.grid.iter() {
            match cell {
                Cell::Operational => {
                    if cur_group > 0 {
                        computed_groups.push(cur_group);
                    }
                    cur_group = 0;
                }
                Cell::Damaged => {
                    cur_group += 1;
                }
                Cell::Unknown => return false,
            }
        }

        if cur_group > 0 {
            computed_groups.push(cur_group);
        }

        self.groups == computed_groups
    }

    fn missing_damaged(&self) -> usize {
        let total = self.groups.iter().sum::<usize>();

        total
            - self
                .grid
                .iter()
                .filter(|cell| **cell == Cell::Damaged)
                .count()
    }

    fn count_unknowns(&self) -> usize {
        self.grid
            .iter()
            .filter(|cell| **cell == Cell::Unknown)
            .count()
    }

    fn with_replacements_for_unknown(&self, replacements: &[Cell]) -> Self {
        let mut replacements = replacements.iter();

        let mut new_grid = Vec::new();

        for cell in self.grid.iter() {
            match cell {
                Cell::Unknown => {
                    if let Some(new_cell) = replacements.next() {
                        new_grid.push(*new_cell);
                    } else {
                        new_grid.push(Cell::Unknown);
                    }
                }
                _ => new_grid.push(*cell),
            }
        }

        Self {
            grid: new_grid,
            groups: self.groups.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Cell {
    Operational,
    Damaged,
    Unknown,
}

struct VariantInterator {
    curr: usize,
    length: u32,
    missing: u32,
}

impl Iterator for VariantInterator {
    type Item = Vec<Cell>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.curr >= 2usize.pow(self.length) {
                return None;
            }

            if self.curr.count_ones() == self.missing {
                break;
            }

            self.curr += 1;
        }

        let res = self.gen(self.curr);

        self.curr += 1;

        Some(res)
    }
}

fn variants(length: usize, missing: usize) -> impl Iterator<Item = Vec<Cell>> {
    VariantInterator {
        curr: 0,
        length: length as u32,
        missing: missing as u32,
    }
}

impl VariantInterator {
    fn gen(&self, num: usize) -> Vec<Cell> {
        let mut res = Vec::with_capacity(self.length as usize);
        for i in (0..self.length).rev() {
            if num & (1 << i) != 0 {
                res.push(Cell::Damaged);
            } else {
                res.push(Cell::Operational);
            }
        }

        res
    }
}

fn arrangements(prefix: Vec<Line>, submaps: &[Vec<Cell>], groups: &[usize]) -> Vec<Vec<Line>> {
    // println!(
    //     "prefix: {:?} submaps: {:?}, groups: {:?}",
    //     prefix, submaps, groups
    // );
    if submaps.is_empty() || groups.is_empty() {
        if !groups.is_empty()
            || !submaps
                .iter()
                .all(|sm| sm.iter().all(|c| *c == Cell::Unknown))
        {
            return vec![];
        } else {
            // println!("Found a valid solution");
            return vec![prefix];
        }
    }

    let mut res = Vec::new();

    let next_group = &submaps[0];
    let current_damaged = next_group
        .iter()
        .filter(|cell| **cell == Cell::Damaged)
        .count();
    let max_len = next_group.len();
    for taken_groups in 0..=groups.len() {
        let taken = &groups[..taken_groups];
        let damaged = taken.iter().sum::<usize>();
        let space_required = if taken.is_empty() {
            0usize
        } else {
            damaged + taken.len() - 1usize
        };
        if current_damaged > damaged || space_required > max_len {
            continue;
        }

        // This is a potentially valid prefix, build it and recurse to check if we have to add it

        let new_groups = &groups[taken_groups..];
        let new_submaps = &submaps[1..];
        let mut new_prefix = prefix.clone();
        new_prefix.push(Line {
            grid: next_group.clone(),
            groups: taken.to_vec(),
        });
        res.append(&mut arrangements(new_prefix, new_submaps, new_groups));
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arrangements() {
        let line: Line = ".# 1".parse().unwrap();
        assert_eq!(
            line.potentials(),
            vec![vec![Line {
                grid: vec![Cell::Damaged],
                groups: vec![1]
            }]]
        );

        let line: Line = "???.### 1,1,3".parse().unwrap();
        assert_eq!(
            line.potentials(),
            vec![vec![
                Line {
                    grid: vec![Cell::Unknown, Cell::Unknown, Cell::Unknown],
                    groups: vec![1, 1],
                },
                Line {
                    grid: vec![Cell::Damaged, Cell::Damaged, Cell::Damaged],
                    groups: vec![3],
                }
            ]]
        );

        let line: Line = "???#???#.?#?????.# 5,1,1,1,2,1"
            .parse::<Line>()
            .unwrap()
            .multiplied(5);
        assert_eq!(line.potentials().len(), 1);

        let line: Line = "???#????.?? 3,1,1".parse::<Line>().unwrap();
        assert_eq!(line.potentials().len(), 2);

        let line: Line = "????###??.????#.# 5,1,2,1".parse().unwrap();
        assert_eq!(line.potentials().len(), 2);
    }

    #[test]
    fn test_solve_line() {
        let mut cache = HashMap::new();
        let line: Line = ".# 1".parse().unwrap();
        assert_eq!(solve_line(&line, &mut cache), 1);
        let mut cache = HashMap::new();
        let line: Line = "???.### 1,1,3".parse().unwrap();
        assert_eq!(solve_line(&line, &mut cache), 1);
    }

    #[test]
    fn test_remove_prefix() {
        let line: Line = ".# 1".parse().unwrap();
        assert_eq!(
            line.remove_prefix(),
            Line {
                grid: vec![Cell::Damaged],
                groups: vec![1]
            }
        );

        let line: Line = ".#.???.#?#?#??##??. 1,1,5,4".parse().unwrap();
        let without_prefix: Line = "???.#?#?#??##??. 1,5,4".parse().unwrap();
        assert_eq!(line.remove_prefix(), without_prefix);

        let line: Line = "#?? 1,1".parse().unwrap();
        let without_prefix: Line = "#?? 1,1".parse().unwrap();
        assert_eq!(line.remove_prefix(), without_prefix);
    }
}
