use anyhow::Result;
use std::{
    collections::HashSet,
    fmt::{Display, Write},
    str::FromStr,
};

fn main() {
    solve(include_str!("../input.txt")).unwrap();
}

fn solve(input: &str) -> Result<()> {
    let maps = input
        .split("\n\n")
        .map(|map| map.parse::<Map>())
        .collect::<Result<Vec<_>, anyhow::Error>>()?;
    println!(
        "Part 1: {}",
        maps.iter()
            .filter_map(|m| m.find_reflection())
            .map(|m| m.score())
            .sum::<i64>()
    );
    println!(
        "Part 2: {}",
        maps.iter()
            .map(|m| m.find_reflections_with_mistake().unwrap())
            .map(|m| m.score())
            .sum::<i64>()
    );
    Ok(())
}

#[derive(Debug)]
struct Map {
    rocks: HashSet<Pos>,
    width: i64,
    height: i64,
}

impl Map {
    fn in_bounds(&self, pos: Pos) -> bool {
        pos.x >= 1 && pos.x <= self.height && pos.y >= 1 && pos.y <= self.width
    }

    fn find_reflections_with_mistake(&self) -> Option<Mirror> {
        for i in 1..self.width {
            let m = Mirror::Vertical { col: i };
            let (before, after): (HashSet<&Pos>, HashSet<&Pos>) =
                self.rocks.iter().partition(|p| p.y <= i);

            let mut mistakes = 0;

            for x in 1..=self.height {
                for y in 1..=i {
                    let p = Pos { x, y };
                    let reflected = m.reflected(p);

                    if !self.in_bounds(reflected) {
                        continue;
                    }

                    if before.contains(&p) != after.contains(&reflected) {
                        mistakes += 1;
                    }
                }
            }

            if mistakes == 1 {
                return Some(m);
            }
        }

        for i in 1..self.height {
            let m = Mirror::Horizontal { row: i };
            let (before, after): (HashSet<&Pos>, HashSet<&Pos>) =
                self.rocks.iter().partition(|p| p.x <= i);

            let mut mistakes = 0;

            for x in 1..=i {
                for y in 1..=self.width {
                    let p = Pos { x, y };
                    let reflected = m.reflected(p);

                    if !self.in_bounds(reflected) {
                        continue;
                    }

                    if before.contains(&p) != after.contains(&reflected) {
                        mistakes += 1;
                    }
                }
            }

            if mistakes == 1 {
                return Some(m);
            }
        }

        None
    }

    fn find_reflection(&self) -> Option<Mirror> {
        'mirror: for i in 1..self.width {
            let m = Mirror::Vertical { col: i };
            let (before, mut after): (HashSet<&Pos>, HashSet<&Pos>) =
                self.rocks.iter().partition(|p| p.y <= i);
            for item in before {
                let reflected = m.reflected(*item);

                if !self.in_bounds(reflected) {
                    continue;
                }

                if !after.remove(&reflected) {
                    continue 'mirror;
                }
            }

            if after.is_empty() || after.iter().all(|p| p.y > 2 * i) {
                return Some(m);
            }
        }

        'mirror: for i in 1..self.height {
            let m = Mirror::Horizontal { row: i };
            let (before, mut after): (HashSet<&Pos>, HashSet<&Pos>) =
                self.rocks.iter().partition(|p| p.x <= i);

            for item in before {
                let reflected = m.reflected(*item);

                if !self.in_bounds(reflected) {
                    continue;
                }

                if !after.remove(&reflected) {
                    continue 'mirror;
                }
            }

            if after.is_empty() || after.iter().all(|p| p.x > 2 * i) {
                return Some(m);
            }
        }

        None
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 1..=self.height {
            for y in 1..=self.width {
                if self.rocks.contains(&Pos { x, y }) {
                    f.write_char('#')?;
                } else {
                    f.write_char('.')?;
                }
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mirror {
    Horizontal { row: i64 },
    Vertical { col: i64 },
}

impl Mirror {
    fn score(&self) -> i64 {
        match self {
            Mirror::Horizontal { row } => *row * 100,
            Mirror::Vertical { col } => *col,
        }
    }
}

impl Mirror {
    fn reflected(&self, pos: Pos) -> Pos {
        match self {
            Mirror::Horizontal { row } => {
                if pos.x < *row {
                    Pos {
                        x: *row + (*row - pos.x) + 1,
                        y: pos.y,
                    }
                } else {
                    Pos {
                        x: *row - (pos.x - *row) + 1,
                        y: pos.y,
                    }
                }
            }
            Mirror::Vertical { col } => {
                if pos.y < *col {
                    Pos {
                        x: pos.x,
                        y: *col + (*col - pos.y) + 1,
                    }
                } else {
                    Pos {
                        x: pos.x,
                        y: *col - (col.abs_diff(pos.y) as i64) + 1,
                    }
                }
            }
        }
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut width = 0i64;
        let mut height = 0i64;
        let mut rocks = HashSet::new();
        for (x, row) in s.lines().enumerate() {
            height = x as i64 + 1;
            width = width.max(row.len() as i64);
            for (y, cell) in row.char_indices() {
                if cell == '#' {
                    rocks.insert(Pos {
                        x: x as i64 + 1,
                        y: y as i64 + 1,
                    });
                }
            }
        }
        Ok(Self {
            width,
            height,
            rocks,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Map, Mirror, Pos};

    #[test]
    fn test_mirror() {
        let v = Mirror::Vertical { col: 5 };
        assert_eq!(Pos { x: 1, y: 9 }, v.reflected(Pos { x: 1, y: 2 }));
        assert_eq!(Pos { x: 1, y: 2 }, v.reflected(Pos { x: 1, y: 9 }));
        assert_eq!(Pos { x: 1, y: 8 }, v.reflected(Pos { x: 1, y: 3 }));
        let h = Mirror::Horizontal { row: 4 };
        assert_eq!(Pos { x: 2, y: 1 }, h.reflected(Pos { x: 7, y: 1 }));
        assert_eq!(Pos { x: 7, y: 1 }, h.reflected(Pos { x: 2, y: 1 }));
        assert_eq!(Pos { x: 5, y: 1 }, h.reflected(Pos { x: 4, y: 1 }));
        assert_eq!(Pos { x: 4, y: 1 }, h.reflected(Pos { x: 5, y: 1 }));
    }

    #[test]
    fn test_find_reflection() {
        let m: Map = r#".####..#.#.#.##..
........#..##....
..##..#.....#..##
......##.##.#####
######.#.####....
..##....#..##.#..
.#..#..#####.#...
..##...#..#...#.#
#######.#....####"#
            .parse()
            .unwrap();

        assert_eq!(Some(Mirror::Vertical { col: 3 }), m.find_reflection());
    }

    #[test]
    fn test_find_reflection_with_smudge() {
        let m: Map = r#"###.#...#.#.##.#.
#.#...####...#..#
#.#...####...#..#
###.#...#.#.##.#.
..####..#####....
.....###...#..###
.....###...#..###
..####..#####....
###.#...#.#.##.##"#
            .parse()
            .unwrap();
        assert_eq!(
            Some(Mirror::Horizontal { row: 6 }),
            m.find_reflections_with_mistake()
        );

        let m: Map = r#"......#
......#
..#..#.
####.#.
..#.##.
##.##..
..#.###
...###.
##.###.
...###.
..#...#
##...#.
##...#.
####.##
##.#.##
###..#.
.#.##.#"#
            .parse()
            .unwrap();
        assert_eq!(
            Some(Mirror::Vertical { col: 1 }),
            m.find_reflections_with_mistake()
        );
    }
}
