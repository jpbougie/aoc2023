use std::{
    collections::{HashMap, HashSet},
    num::ParseIntError,
    str::FromStr,
};

use regex::Regex;

fn main() {
    solve(include_str!("../input.txt")).unwrap();
}

#[derive(Debug)]
struct Card {
    id: u32,
    winning: HashSet<u32>,
    numbers: HashSet<u32>,
}

impl Card {
    fn matches(&self) -> usize {
        self.winning.intersection(&self.numbers).count()
    }
    fn score_part1(&self) -> u64 {
        let count = self.winning.intersection(&self.numbers).count() as u32;
        if count == 0 {
            return 0;
        }
        2u64.pow(count - 1)
    }
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id_part, nums_part) = s.split_once(": ").unwrap();
        let num_re = Regex::new(r#"\d+"#).unwrap();
        let id = num_re.find(id_part).unwrap().as_str().parse::<u32>()?;

        let (winning, numbers) = nums_part.split_once(" | ").unwrap();
        let winning = num_re
            .find_iter(winning)
            .map(|m| m.as_str().parse::<u32>())
            .collect::<Result<HashSet<_>, ParseIntError>>()?;
        let numbers = num_re
            .find_iter(numbers)
            .map(|m| m.as_str().parse::<u32>())
            .collect::<Result<HashSet<_>, ParseIntError>>()?;

        Ok(Card {
            id,
            winning,
            numbers,
        })
    }
}

#[derive(Debug)]
struct Input {
    cards: Vec<Card>,
    counts: HashMap<u32, usize>,
}

impl Input {
    fn part1(&self) -> u64 {
        self.cards.iter().map(|c| c.score_part1()).sum()
    }

    fn part2(&mut self) -> usize {
        for card in self.cards.iter() {
            let wins = card.matches() as u32;
            let self_count = *self.counts.get(&card.id).unwrap();
            for id in (card.id + 1)..=(card.id + wins) {
                let ent = self.counts.entry(id).or_default();
                *ent += self_count;
            }
        }

        self.counts.values().sum()
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s
            .lines()
            .map(|l| l.parse::<Card>())
            .collect::<Result<Vec<_>, anyhow::Error>>()?;

        let counts = cards.iter().map(|c| (c.id, 1)).collect();
        Ok(Input { cards, counts })
    }
}

fn solve(input: &str) -> Result<(), anyhow::Error> {
    let mut input: Input = input.parse()?;
    println!("Part 1: {}", input.part1());
    println!("Part 2: {}", input.part2());
    Ok(())
}
