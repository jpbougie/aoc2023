fn main() {
    solve(include_str!("../input.txt"));
}

#[derive(Default, Debug)]
struct Pick {
    red: u32,
    green: u32,
    blue: u32,
}

impl Pick {
    fn valid_part1(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn solve(input: &str) {
    let games = input
        .lines()
        .map(|line| {
            let (game, picks) = line.split_once(": ").unwrap();
            let game_id = game.split_once(' ').unwrap().1.parse::<u32>().unwrap();
            let picks: Vec<Pick> = picks
                .split("; ")
                .map(|pick| {
                    let cubes = pick
                        .split(", ")
                        .map(|c| {
                            let (count, color) = c.split_once(' ').unwrap();
                            (count.parse().unwrap(), color)
                        })
                        .collect::<Vec<(u32, &str)>>();
                    let mut pick = Pick::default();
                    for (count, color) in cubes {
                        match color {
                            "red" => pick.red += count,
                            "green" => pick.green += count,
                            "blue" => pick.blue += count,
                            _ => panic!("Unexpected color {color}"),
                        };
                    }

                    pick
                })
                .collect();

            (game_id, picks)
        })
        .collect::<Vec<_>>();

    println!(
        "Part 01: {}",
        games
            .iter()
            .filter_map(|(id, picks)| if picks.iter().all(|p| p.valid_part1()) {
                Some(id)
            } else {
                None
            })
            .sum::<u32>()
    );

    let power: u32 = games
        .iter()
        .map(|(_, picks)| {
            let mut min_pick = Pick::default();
            for pick in picks.iter() {
                if pick.red > min_pick.red {
                    min_pick.red = pick.red;
                }

                if pick.green > min_pick.green {
                    min_pick.green = pick.green;
                }

                if pick.blue > min_pick.blue {
                    min_pick.blue = pick.blue;
                }
            }
            min_pick.power()
        })
        .sum();

    println!("Part 2: {power}");
}
