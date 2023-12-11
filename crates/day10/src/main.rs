use std::{
    collections::{BinaryHeap, HashSet},
    fmt::Display,
    io::Write,
    str::FromStr,
};

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use anyhow::{Ok, Result};
fn main() {
    solve(include_str!("../input.txt")).unwrap();
}

fn solve(input: &str) -> Result<()> {
    do_solve(input, Cell::Vertical)?;
    Ok(())
}

fn do_solve(input: &str, starting_cell: Cell) -> Result<()> {
    let mut game: Labyrinth = input.parse()?;
    let starting = game.starting_position().unwrap();
    game.map[starting.x as usize][starting.y as usize] =
        Cell::Starting(Some(Box::new(starting_cell.clone())));

    let pipes = game.connected();
    let max = pipes.iter().map(|x| x.0).max().unwrap();
    println!("Part 1: {}", max);
    game.map[starting.x as usize][starting.y as usize] = starting_cell;

    let (minx, miny, maxx, maxy) = pipes.iter().fold(
        (i64::MAX, i64::MAX, 0, 0),
        |(minx, miny, maxx, maxy), (_, pos)| {
            (
                minx.min(pos.x),
                miny.min(pos.y),
                maxx.max(pos.x),
                maxy.max(pos.y),
            )
        },
    );

    let edges = pipes.iter().map(|(_, pos)| *pos).collect::<HashSet<Pos>>();

    let mut inside_count = 0;
    let mut inside_nodes = HashSet::new();
    for x in minx..=maxx {
        let mut inside = false;
        for y in miny..=maxy {
            let pos = Pos { x, y };
            if edges.contains(&pos) {
                if matches!(game.at(pos), Cell::Vertical | Cell::UpLeft | Cell::UpRight) {
                    inside = !inside;
                }
            } else if inside {
                inside_nodes.insert(pos);
                inside_count += 1;
            }
        }
    }

    print_labyrinth(&game, &edges, &inside_nodes)?;
    println!("Part 2: {}", inside_count);
    Ok(())
}

fn print_labyrinth(lab: &Labyrinth, edge: &HashSet<Pos>, inside: &HashSet<Pos>) -> Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    let mut edge_color = ColorSpec::new();
    edge_color.set_fg(Some(Color::Green));
    let mut inside_color = ColorSpec::new();
    inside_color.set_fg(Some(Color::Red));

    for (x, row) in lab.map.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            let pos = Pos {
                x: x as i64,
                y: y as i64,
            };
            if edge.contains(&pos) {
                stdout.set_color(&edge_color)?;
            }
            if inside.contains(&pos) {
                stdout.set_color(&inside_color)?;
            }
            write!(&mut stdout, "{}", cell)?;
            stdout.reset()?;
        }
        writeln!(&mut stdout)?;
    }

    Ok(())
}

struct Labyrinth {
    map: Vec<Vec<Cell>>,
}

impl Labyrinth {
    fn at(&self, p: Pos) -> &Cell {
        &self.map[p.x as usize][p.y as usize]
    }

    fn connected(&self) -> Vec<(usize, Pos)> {
        let mut visited = HashSet::new();
        let mut res = Vec::new();
        let mut to_visit = BinaryHeap::new();
        let starting = self.starting_position().unwrap();

        to_visit.push(Step {
            steps: 0,
            pos: starting,
        });
        while let Some(step) = to_visit.pop() {
            // println!("Visiting {:?}", step.pos);
            visited.insert(step.pos);
            res.push((step.steps, step.pos));

            for n in self.at(step.pos).next(step.pos) {
                if !visited.contains(&n) {
                    to_visit.push(Step {
                        steps: step.steps + 1,
                        pos: n,
                    });
                }
            }
        }

        res
    }

    fn starting_position(&self) -> Option<Pos> {
        for (x, rows) in self.map.iter().enumerate() {
            for (y, cell) in rows.iter().enumerate() {
                if matches!(cell, Cell::Starting(_)) {
                    return Some(Pos {
                        x: x as i64,
                        y: y as i64,
                    });
                }
            }
        }

        None
    }
}

#[derive(PartialEq, Eq)]
struct Step {
    steps: usize,
    pos: Pos,
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .steps
            .cmp(&self.steps)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn up(&self) -> Self {
        Pos {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn down(&self) -> Self {
        Pos {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn left(&self) -> Self {
        Pos {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn right(&self) -> Self {
        Pos {
            x: self.x,
            y: self.y + 1,
        }
    }
}

impl FromStr for Labyrinth {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        Ok(Self {
            map: s
                .lines()
                .map(|line| {
                    line.chars()
                        .map(TryInto::try_into)
                        .collect::<Result<Vec<Cell>>>()
                })
                .collect::<Result<Vec<Vec<Cell>>>>()?,
        })
    }
}

#[derive(Clone)]
enum Cell {
    Ground,
    Starting(Option<Box<Cell>>),
    Vertical,
    Horizontal,
    UpRight,
    UpLeft,
    DownLeft,
    DownRight,
}

impl Cell {
    fn next(&self, p: Pos) -> Vec<Pos> {
        match self {
            Cell::Ground => vec![],
            Cell::Starting(ref x) => {
                if let Some(x) = x {
                    x.next(p)
                } else {
                    vec![]
                }
            }
            Cell::Vertical => vec![p.up(), p.down()],
            Cell::Horizontal => vec![p.left(), p.right()],
            Cell::UpRight => vec![p.up(), p.right()],
            Cell::UpLeft => vec![p.up(), p.left()],
            Cell::DownLeft => vec![p.down(), p.left()],
            Cell::DownRight => vec![p.down(), p.right()],
        }
    }
}

impl TryFrom<char> for Cell {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::prelude::v1::Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Ground),
            'S' => Ok(Self::Starting(None)),
            '|' => Ok(Self::Vertical),
            '-' => Ok(Self::Horizontal),
            'L' => Ok(Self::UpRight),
            'J' => Ok(Self::UpLeft),
            '7' => Ok(Self::DownLeft),
            'F' => Ok(Self::DownRight),
            x => Err(anyhow::anyhow!("Unknown cell {}", x)),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        f.write_char(match self {
            Cell::Ground => '.',
            Cell::Starting(_) => 'S',
            Cell::Vertical => '|',
            Cell::Horizontal => '-',
            Cell::UpRight => 'L',
            Cell::UpLeft => 'J',
            Cell::DownLeft => '7',
            Cell::DownRight => 'F',
        })
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
