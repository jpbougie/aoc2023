use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

use anyhow::Result;
fn main() {
    solve(include_str!("../input.txt")).unwrap();
}

fn solve(input: &str) -> Result<()> {
    let map: Map = input.parse()?;

    let p1 = part1(&map, Pos { x: 0, y: -1 }, Dir::East);
    println!("Part 1: {p1}");

    let part2 = *[
        (0..map.width)
            .map(|y| part1(&map, Pos { x: -1, y: y as i64 }, Dir::South))
            .max()
            .unwrap_or_default(),
        (0..map.width)
            .map(|y| {
                part1(
                    &map,
                    Pos {
                        x: map.height as i64,
                        y: y as i64,
                    },
                    Dir::North,
                )
            })
            .max()
            .unwrap_or_default(),
        (0..map.height)
            .map(|x| part1(&map, Pos { x: x as i64, y: -1 }, Dir::East))
            .max()
            .unwrap_or_default(),
        (0..map.height)
            .map(|x| {
                part1(
                    &map,
                    Pos {
                        x: x as i64,
                        y: map.width as i64,
                    },
                    Dir::West,
                )
            })
            .max()
            .unwrap_or_default(),
    ]
    .iter()
    .max()
    .unwrap();

    println!("Part 2: {}", part2);
    Ok(())
}

fn part1(map: &Map, start: Pos, dir: Dir) -> usize {
    let mut energized = HashSet::new();
    let mut visited = HashSet::new();
    let mut heads = VecDeque::new();
    heads.push_back(Ray { pos: start, dir });

    while let Some(head) = heads.pop_front() {
        energized.insert(head.pos);
        if visited.contains(&head) {
            continue;
        }
        visited.insert(head);
        let n = head.pos.next_cell(head.dir);
        if !map.within_bounds(n) {
            continue;
        }

        if let Some(item) = map.items.get(&n) {
            let new_heads = map.apply_item(item, n, head.dir);
            for head in new_heads {
                heads.push_back(head);
            }
        } else {
            heads.push_back(Ray {
                pos: n,
                dir: head.dir,
            });
        }
    }

    energized.remove(&start);
    energized.len()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Ray {
    pos: Pos,
    dir: Dir,
}

enum Cell {
    HorizontalSplitter, // -
    VerticalSplitter,   // |
    LeftLeaning,        // \
    RightLeaning,       // /
}

struct Map {
    items: HashMap<Pos, Cell>,
    width: usize,
    height: usize,
}

impl Map {
    fn display_energized(&self, energized: &HashSet<Pos>) {
        for x in 0..self.height {
            for y in 0..self.width {
                if energized.contains(&Pos {
                    x: x as i64,
                    y: y as i64,
                }) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
    fn within_bounds(&self, pos: Pos) -> bool {
        pos.x >= 0 && pos.x < self.height as i64 && pos.y >= 0 && pos.y < self.width as i64
    }

    fn apply_item(&self, item: &Cell, pos: Pos, going: Dir) -> Vec<Ray> {
        match item {
            Cell::HorizontalSplitter => match going {
                Dir::North | Dir::South => {
                    vec![
                        Ray {
                            pos,
                            dir: Dir::West,
                        },
                        Ray {
                            pos,
                            dir: Dir::East,
                        },
                    ]
                }
                _ => vec![Ray { pos, dir: going }],
            },
            Cell::VerticalSplitter => match going {
                Dir::North | Dir::South => vec![Ray { pos, dir: going }],
                Dir::West | Dir::East => vec![
                    Ray {
                        pos,
                        dir: Dir::North,
                    },
                    Ray {
                        pos,
                        dir: Dir::South,
                    },
                ],
            },
            Cell::LeftLeaning => match going {
                // \
                Dir::North => vec![Ray {
                    pos,
                    dir: Dir::West,
                }],
                Dir::South => vec![Ray {
                    pos,
                    dir: Dir::East,
                }],
                Dir::East => vec![Ray {
                    pos,
                    dir: Dir::South,
                }],
                Dir::West => vec![Ray {
                    pos,
                    dir: Dir::North,
                }],
            },
            Cell::RightLeaning => match going {
                // /
                Dir::North => vec![Ray {
                    pos,
                    dir: Dir::East,
                }],
                Dir::South => vec![Ray {
                    pos,
                    dir: Dir::West,
                }],
                Dir::East => vec![Ray {
                    pos,
                    dir: Dir::North,
                }],
                Dir::West => vec![Ray {
                    pos,
                    dir: Dir::South,
                }],
            },
        }
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut items = HashMap::new();
        let mut height = 0;
        let mut width = 0;
        for (x, s) in s.lines().enumerate() {
            height += 1;
            width = s.len();
            for (y, ch) in s.char_indices() {
                let item = match ch {
                    '|' => Some(Cell::VerticalSplitter),
                    '-' => Some(Cell::HorizontalSplitter),
                    '\\' => Some(Cell::LeftLeaning),
                    '/' => Some(Cell::RightLeaning),
                    _ => None,
                };

                if let Some(item) = item {
                    items.insert(
                        Pos {
                            x: x as i64,
                            y: y as i64,
                        },
                        item,
                    );
                }
            }
        }

        Ok(Self {
            items,
            height,
            width,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Eq, Hash, Default, Clone, Copy)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn next_cell(&self, dir: Dir) -> Pos {
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
