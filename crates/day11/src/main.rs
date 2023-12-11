use std::{collections::HashSet, str::FromStr};

use anyhow::Result;
fn main() {
    solve(include_str!("../input.txt")).unwrap();
}

fn solve(input: &str) -> Result<()> {
    let base_grid: BaseGrid = input.parse()?;
    let expanded: ExpandedGrid = ExpandedGrid::from(&base_grid, 2);
    let part1 = expanded
        .pairs()
        .into_iter()
        .map(|(a, b)| distance(a, b))
        .sum::<usize>();

    println!("Part 1: {}", part1);

    let mega_expanded: ExpandedGrid = ExpandedGrid::from(&base_grid, 1_000_000);
    let part2 = mega_expanded
        .pairs()
        .into_iter()
        .map(|(a, b)| distance(a, b))
        .sum::<usize>();

    println!("Part 2: {}", part2);
    Ok(())
}

fn distance(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

struct ExpandedGrid {
    galaxies: HashSet<(usize, usize)>,
}

impl ExpandedGrid {
    fn from(value: &BaseGrid, expansion_factor: usize) -> Self {
        // Scan the rows and cols to check which ones are empty.
        let empty_rows: HashSet<usize> = (0..value.height())
            .filter(|row| value.row_iter(*row).all(|c| matches!(c, Cell::Empty)))
            .collect();
        let empty_cols: HashSet<usize> = (0..value.width())
            .filter(|col| value.col_iter(*col).all(|c| matches!(c, Cell::Empty)))
            .collect();

        let mut galaxies = HashSet::new();

        let mut expanded_row = 0;
        for row in 0..value.height() {
            if empty_rows.contains(&row) {
                expanded_row += expansion_factor;
            } else {
                let mut expanded_col = 0;
                for col in 0..value.width() {
                    if matches!(value.at(row, col), &Cell::Galaxy) {
                        galaxies.insert((expanded_row, expanded_col));
                        expanded_col += 1;
                    } else if empty_cols.contains(&col) {
                        expanded_col += expansion_factor;
                    } else {
                        expanded_col += 1;
                    }
                }
                expanded_row += 1;
            }
        }

        Self { galaxies }
    }
}

impl ExpandedGrid {
    fn pairs(&self) -> Vec<((usize, usize), (usize, usize))> {
        let mut res = Vec::new();

        for gal in self.galaxies.iter() {
            for other_gal in self.galaxies.iter() {
                if gal < other_gal {
                    res.push((*gal, *other_gal));
                }
            }
        }

        res
    }
}

#[derive(Debug)]
struct BaseGrid {
    grid: Vec<Vec<Cell>>,
}

impl FromStr for BaseGrid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        Ok(Self {
            grid: s
                .lines()
                .map(|l| {
                    l.chars()
                        .map(|ch| match ch {
                            '#' => Cell::Galaxy,
                            '.' => Cell::Empty,
                            _ => panic!(),
                        })
                        .collect()
                })
                .collect(),
        })
    }
}

impl BaseGrid {
    fn width(&self) -> usize {
        self.grid.first().map(|row| row.len()).unwrap_or(0)
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn row_iter(&self, row: usize) -> impl Iterator<Item = &Cell> {
        self.grid[row].iter()
    }

    fn col_iter(&self, col: usize) -> impl Iterator<Item = &Cell> {
        ColIter {
            col,
            cur: 0,
            grid: self,
        }
    }

    fn at(&self, row: usize, col: usize) -> &Cell {
        &self.grid[row][col]
    }
}

struct ColIter<'a> {
    col: usize,
    cur: usize,
    grid: &'a BaseGrid,
}

impl<'a> Iterator for ColIter<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.grid.grid.len() {
            return None;
        }

        let res = Some(&self.grid.grid[self.cur][self.col]);
        self.cur += 1;
        res
    }
}

#[derive(Debug)]
enum Cell {
    Empty,
    Galaxy,
}
