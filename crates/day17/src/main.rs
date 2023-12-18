use std::{
    collections::{BinaryHeap, HashMap},
    io::Write,
    str::FromStr,
};

use anyhow::Result;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
fn main() {
    solve(include_str!("../input.txt")).unwrap();
}

fn solve(input: &str) -> Result<()> {
    let map: Map = input.parse()?;
    println!("Map is {}x{}", map.width(), map.height());
    let distances = find_distances(
        &map,
        Pos { x: 0, y: 0 },
        Pos {
            x: map.height() - 1,
            y: map.width() - 1,
        },
        |_previous, plan| plan.loc.dir_count <= 3,
    );

    let to = Pos {
        x: map.height() - 1,
        y: map.width() - 1,
    };
    let p1 = distances
        .iter()
        .filter_map(|(l, c)| if l.pos == to { Some(c) } else { None })
        .min()
        .unwrap();
    println!("Part 1: {}", p1);

    let distances = find_distances(
        &map,
        Pos { x: 0, y: 0 },
        Pos {
            x: map.height() - 1,
            y: map.width() - 1,
        },
        |previous, new_plan| {
            if previous.loc.dir != new_plan.loc.dir {
                previous.loc.dir_count >= 4
            } else {
                new_plan.loc.dir_count <= 10
            }
        },
    );

    let to = Pos {
        x: map.height() - 1,
        y: map.width() - 1,
    };
    let p2 = distances
        .iter()
        .filter_map(|(l, c)| if l.pos == to { Some(c) } else { None })
        .min()
        .unwrap();
    println!("Part 2: {}", p2);
    Ok(())
}

#[allow(unused)]
fn print_map_with_path(map: &Map, path: &Plan) -> anyhow::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    let mut path_color = ColorSpec::new();
    path_color.set_fg(Some(Color::Green)).set_bold(true);

    let mut steps = HashMap::new();
    let mut pos = Pos { x: 0, y: 0 };
    for step in path.steps.iter() {
        pos = pos.mv(*step);
        steps.insert(pos, step);
    }

    for x in 0..map.height() {
        for y in 0..map.width() {
            if let Some(dir) = steps.get(&Pos { x, y }) {
                let ch = match dir {
                    Dir::North => '^',
                    Dir::South => 'v',
                    Dir::East => '>',
                    Dir::West => '<',
                };

                stdout.set_color(&path_color)?;
                write!(&mut stdout, "{}", ch)?;
            } else {
                write!(&mut stdout, "{}", map.grid[x as usize][y as usize])?;
            }
            stdout.reset()?;
        }
        writeln!(&mut stdout)?;
    }

    Ok(())
}

fn find_distances<F>(map: &Map, from: Pos, to: Pos, accept_fn: F) -> HashMap<Loc, u32>
where
    F: Fn(&Plan, &Plan) -> bool,
{
    let mut distances = HashMap::new();
    let mut to_visit = BinaryHeap::new();
    to_visit.push(Plan {
        cost: 0,
        loc: Loc {
            pos: from,
            dir: Dir::East,
            dir_count: 0,
        },
        steps: Vec::new(),
        dest: to,
    });
    to_visit.push(Plan {
        cost: 0,
        loc: Loc {
            pos: from,
            dir: Dir::South,
            dir_count: 0,
        },
        dest: to,
        steps: Vec::new(),
    });

    while let Some(current) = to_visit.pop() {
        // if current.loc.pos == to {
        //     println!("{:?}", current);
        //     print_map_with_path(map, &current);
        // }

        if let Some(cost) = distances.get(&current.loc) {
            if *cost <= current.cost {
                continue;
            }
        }

        distances.insert(current.loc, current.cost);

        for dir in [Dir::North, Dir::East, Dir::South, Dir::West] {
            if current.loc.dir.opposite() == dir {
                continue;
            }
            let next_pos = current.loc.pos.mv(dir);
            if let Some(cost) = map.cost(next_pos) {
                let mut steps = current.steps.clone();
                steps.push(dir);
                let new_plan = Plan {
                    cost: current.cost + cost,
                    loc: Loc {
                        pos: next_pos,
                        dir,
                        dir_count: if dir == current.loc.dir {
                            current.loc.dir_count + 1
                        } else {
                            1
                        },
                    },
                    dest: to,
                    steps,
                };

                if accept_fn(&current, &new_plan) {
                    to_visit.push(new_plan);
                }
            }
        }
    }

    distances
}

struct Map {
    grid: Vec<Vec<u32>>,
}

impl Map {
    fn width(&self) -> i64 {
        self.grid[0].len() as i64
    }
    fn height(&self) -> i64 {
        self.grid.len() as i64
    }

    fn within_bounds(&self, pos: &Pos) -> bool {
        pos.x >= 0 && pos.x < self.height() && pos.y >= 0 && pos.y < self.width()
    }

    fn cost(&self, pos: Pos) -> Option<u32> {
        if !self.within_bounds(&pos) {
            return None;
        }

        Some(self.grid[pos.x as usize][pos.y as usize])
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        Ok(Map {
            grid: s
                .lines()
                .map(|l| l.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
                .collect(),
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn mv(&self, dir: Dir) -> Pos {
        match dir {
            Dir::North => Pos {
                x: self.x - 1,
                y: self.y,
            },
            Dir::South => Pos {
                x: self.x + 1,
                y: self.y,
            },
            Dir::East => Pos {
                x: self.x,
                y: self.y + 1,
            },
            Dir::West => Pos {
                x: self.x,
                y: self.y - 1,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Plan {
    cost: u32,
    loc: Loc,
    dest: Pos,
    steps: Vec<Dir>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Loc {
    pos: Pos,
    dir: Dir,
    dir_count: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn opposite(&self) -> Self {
        match self {
            Dir::North => Dir::South,
            Dir::South => Dir::North,
            Dir::East => Dir::West,
            Dir::West => Dir::East,
        }
    }
}

impl Plan {
    fn h(&self) -> u32 {
        // (self.dest.x + self.dest.y - self.pos.x.abs_diff(self.dest.x) as i64
        //     + self.pos.y.abs_diff(self.dest.y) as i64) as u32
        0
    }
}

impl Ord for Plan {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.cost + other.h())
            .cmp(&(self.cost + self.h()))
            .then_with(|| self.loc.cmp(&other.loc))
    }
}

impl PartialOrd for Plan {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
