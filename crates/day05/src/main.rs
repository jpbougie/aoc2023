use std::num::ParseIntError;

fn main() {
    solve(include_str!("../input.txt")).unwrap();
}

#[derive(Debug)]
struct Game {
    seeds: Vec<u64>,
    seed_to_soil: Vec<Mapping>,
    soil_to_fertilizer: Vec<Mapping>,
    fertilizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temperature: Vec<Mapping>,
    temperature_to_humidity: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>,
}

impl Game {
    fn part1(&self) -> u64 {
        self.seeds
            .iter()
            .map(|seed| {
                let mut s = *seed;
                s = apply_map(s, &self.seed_to_soil);
                s = apply_map(s, &self.soil_to_fertilizer);
                s = apply_map(s, &self.fertilizer_to_water);
                s = apply_map(s, &self.water_to_light);
                s = apply_map(s, &self.light_to_temperature);
                s = apply_map(s, &self.temperature_to_humidity);
                s = apply_map(s, &self.humidity_to_location);
                s
            })
            .min()
            .unwrap()
    }

    fn part2(self) -> u64 {
        let mut mappings: Vec<Vec<Transform>> = vec![
            self.seed_to_soil.into_iter().map(Into::into).collect(),
            self.soil_to_fertilizer
                .into_iter()
                .map(Into::into)
                .collect(),
            self.fertilizer_to_water
                .into_iter()
                .map(Into::into)
                .collect(),
            self.water_to_light.into_iter().map(Into::into).collect(),
            self.light_to_temperature
                .into_iter()
                .map(Into::into)
                .collect(),
            self.temperature_to_humidity
                .into_iter()
                .map(Into::into)
                .collect(),
            self.humidity_to_location
                .into_iter()
                .map(Into::into)
                .collect(),
        ];

        mappings.iter_mut().for_each(fill);

        let mut segments = self
            .seeds
            .chunks(2)
            .map(|ch| Segment {
                from: ch[0],
                to: ch[0] + ch[1],
            })
            .collect::<Vec<_>>();

        for map in mappings {
            segments = segments
                .into_iter()
                .flat_map(|f| {
                    map.iter()
                        .filter_map(|m| m.subsequent(f))
                        .collect::<Vec<_>>()
                })
                .collect();
        }

        segments.iter().map(|s| s.from).min().unwrap()
    }
}

fn fill(maps: &mut Vec<Transform>) {
    maps.sort();
    let mut next = 0;
    let mut to_add = Vec::new();
    for m in maps.iter() {
        if next < m.input_range.from {
            to_add.push(Transform {
                input_range: Segment {
                    from: next,
                    to: m.input_range.from,
                },
                output_range: Segment {
                    from: next,
                    to: m.input_range.from,
                },
            });
        }
        next = m.input_range.to;
    }

    if next < u64::MAX {
        to_add.push(Transform {
            input_range: Segment {
                from: next,
                to: u64::MAX,
            },
            output_range: Segment {
                from: next,
                to: u64::MAX,
            },
        });
    }

    maps.append(&mut to_add);

    maps.sort();
}

fn apply_map(seed: u64, mapping: &[Mapping]) -> u64 {
    for m in mapping {
        if let Some(to) = m.map_seed(seed) {
            return to;
        }
    }

    seed
}

#[derive(Debug)]
struct Mapping {
    from: u64,
    to: u64,
    length: u64,
}

impl Mapping {
    fn map_seed(&self, seed: u64) -> Option<u64> {
        if seed >= self.from && seed < self.from + self.length {
            Some(seed - self.from + self.to)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Segment {
    from: u64,
    to: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Transform {
    input_range: Segment,
    output_range: Segment,
}

impl Transform {
    fn diff(&self) -> i64 {
        self.output_range.from as i64 - self.input_range.from as i64
    }

    fn subsequent(&self, input: Segment) -> Option<Segment> {
        let diff = self.diff();
        if input.from < self.input_range.from {
            if input.to < self.input_range.from {
                return None;
            }
            // Anything smaller than self.input_range.from will be dropped
            Some(Segment {
                from: self.input_range.from.checked_add_signed(diff).unwrap(),
                to: input
                    .to
                    .min(self.input_range.to)
                    .checked_add_signed(diff)
                    .unwrap(),
            })
        } else {
            if input.from >= self.input_range.to {
                return None;
            }
            Some(Segment {
                from: input.from.checked_add_signed(diff).unwrap(),
                to: input
                    .to
                    .min(self.input_range.to)
                    .checked_add_signed(diff)
                    .unwrap(),
            })
        }
    }
}

impl From<Mapping> for Transform {
    fn from(value: Mapping) -> Self {
        Self {
            input_range: Segment {
                from: value.from,
                to: value.from + value.length,
            },
            output_range: Segment {
                from: value.to,
                to: value.to + value.length,
            },
        }
    }
}

fn parse_mapping(input: &str) -> Result<Vec<Mapping>, anyhow::Error> {
    Ok(input
        .lines()
        .skip(1)
        .map(|l| {
            let mut parts = l.split_whitespace();
            let to = parts.next().unwrap().parse::<u64>()?;
            let from = parts.next().unwrap().parse::<u64>()?;
            let length = parts.next().unwrap().parse::<u64>()?;
            Ok(Mapping { from, to, length })
        })
        .collect::<Result<Vec<_>, ParseIntError>>()?)
}

fn parse(input: &str) -> Result<Game, anyhow::Error> {
    let mut parts = input.split("\n\n");
    let seeds_part = parts.next().unwrap();
    let seeds = seeds_part
        .split_once(": ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>())
        .collect::<Result<Vec<u64>, ParseIntError>>()?;

    let seed_to_soil = parse_mapping(parts.next().unwrap())?;
    let soil_to_fertilizer = parse_mapping(parts.next().unwrap())?;
    let fertilizer_to_water = parse_mapping(parts.next().unwrap())?;
    let water_to_light = parse_mapping(parts.next().unwrap())?;
    let light_to_temperature = parse_mapping(parts.next().unwrap())?;
    let temperature_to_humidity = parse_mapping(parts.next().unwrap())?;
    let humidity_to_location = parse_mapping(parts.next().unwrap())?;
    Ok(Game {
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    })
}

fn solve(input: &str) -> Result<(), anyhow::Error> {
    let game = parse(input)?;
    println!("Part 1: {}", game.part1());
    println!("Part 2: {}", game.part2());
    Ok(())
}
