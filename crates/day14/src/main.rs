use std::{
    collections::HashSet,
    fmt::{Display, Write},
    str::FromStr,
};

use anyhow::Result;
use derivative::Derivative;
fn main() {
    solve(include_str!("../input.txt")).unwrap();
}

fn solve(input: &str) -> Result<()> {
    let mut map: Map = input.parse()?;
    let mut part1_map = map.clone();
    part1_map.tilt_north();
    println!("Part 1: {}", part1_map.score());

    println!("part 2: {}", part2(map));

    Ok(())
}

fn part2(mut map: Map) -> usize {
    let mut previous: HashSet<Cycle> = HashSet::new();
    let mut l = 0;
    let mut prev = None;
    loop {
        for dir in [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ] {
            map.apply_direction(dir);
        }
        l += 1;
        let m = map.clone();
        let cycle = Cycle { map: m, len: l };
        if previous.contains(&cycle) {
            println!("Cycle found after {} steps", l);
            prev = previous.take(&cycle);
            break;
        }
        previous.insert(cycle);
    }

    let Some(prev) = prev else { return 0 };

    let cycle_len = l - prev.len;

    let missing_steps = (1000000000 - l) % cycle_len;

    for _ in 0..missing_steps {
        for dir in [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ] {
            map.apply_direction(dir);
        }
    }
    map.score()
}

#[derive(Derivative)]
#[derivative(PartialEq, Hash, Eq, Debug)]
struct Cycle {
    map: Map,
    #[derivative(PartialEq = "ignore")]
    #[derivative(Hash = "ignore")]
    len: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Map {
    map: Vec<Vec<Cell>>,
}

impl Map {
    fn apply_direction(&mut self, dir: Direction) {
        match dir {
            Direction::North => self.tilt_north(),
            Direction::West => self.tilt_west(),
            Direction::South => self.tilt_south(),
            Direction::East => self.tilt_east(),
        }
    }

    fn tilt_north(&mut self) {
        let rows = self.map.len();
        let cols = self.map[0].len();

        for x in 1..rows {
            for y in 0..cols {
                if self.map[x][y] != Cell::RoundRock {
                    continue;
                }

                let mut new_x = None;
                for possible_x in (0..x).rev() {
                    if self.map[possible_x][y] == Cell::Ground {
                        new_x = Some(possible_x);
                    } else {
                        break;
                    }
                }

                if let Some(new_x) = new_x {
                    self.map[x][y] = Cell::Ground;
                    self.map[new_x][y] = Cell::RoundRock;
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        let rows = self.map.len();
        let cols = self.map[0].len();

        for x in (0..(rows - 1)).rev() {
            for y in 0..cols {
                if self.map[x][y] != Cell::RoundRock {
                    continue;
                }

                let mut new_x = None;
                for possible_x in (x + 1)..rows {
                    if self.map[possible_x][y] == Cell::Ground {
                        new_x = Some(possible_x);
                    } else {
                        break;
                    }
                }

                if let Some(new_x) = new_x {
                    self.map[x][y] = Cell::Ground;
                    self.map[new_x][y] = Cell::RoundRock;
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        let rows = self.map.len();
        let cols = self.map[0].len();

        for y in (0..(cols - 1)).rev() {
            for x in 0..rows {
                if self.map[x][y] != Cell::RoundRock {
                    continue;
                }

                let mut new = None;
                for possible in (y + 1)..cols {
                    if self.map[x][possible] == Cell::Ground {
                        new = Some(possible);
                    } else {
                        break;
                    }
                }

                if let Some(new) = new {
                    self.map[x][y] = Cell::Ground;
                    self.map[x][new] = Cell::RoundRock;
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        let rows = self.map.len();
        let cols = self.map[0].len();

        for y in 1..cols {
            for x in 0..rows {
                if self.map[x][y] != Cell::RoundRock {
                    continue;
                }

                let mut new = None;
                for possible in (0..y).rev() {
                    if self.map[x][possible] == Cell::Ground {
                        new = Some(possible);
                    } else {
                        break;
                    }
                }

                if let Some(new) = new {
                    self.map[x][y] = Cell::Ground;
                    self.map[x][new] = Cell::RoundRock;
                }
            }
        }
    }

    fn score(&self) -> usize {
        let max_value_per_row = self.map.len();

        let mut score = 0;
        for (x, row) in self.map.iter().enumerate() {
            score +=
                row.iter().filter(|c| **c == Cell::RoundRock).count() * (max_value_per_row - x);
        }

        score
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        Ok(Map {
            map: s
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|ch| match ch {
                            '#' => Ok(Cell::SquareRock),
                            'O' => Ok(Cell::RoundRock),
                            '.' => Ok(Cell::Ground),
                            _ => Err(anyhow::anyhow!("Unrecognized char {ch}")),
                        })
                        .collect::<Result<Vec<Cell>>>()
                })
                .collect::<Result<Vec<Vec<Cell>>>>()?,
        })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.map.iter() {
            for cell in row.iter() {
                f.write_char(match cell {
                    Cell::Ground => '.',
                    Cell::RoundRock => 'O',
                    Cell::SquareRock => '#',
                })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Cell {
    Ground,
    RoundRock,
    SquareRock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn next(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
}
