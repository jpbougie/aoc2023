use std::num::ParseIntError;

use anyhow::Result;
fn main() {
    solve(include_str!("../input.txt")).unwrap();
}

fn solve(input: &str) -> Result<()> {
    let game: Vec<Vec<i64>> = input
        .lines()
        .map(|l| {
            l.split(' ')
                .map(|num| num.parse::<i64>())
                .collect::<Result<Vec<i64>, ParseIntError>>()
        })
        .collect::<Result<_, ParseIntError>>()?;

    let score = part1(&game);
    println!("Part 1: {}", score);
    let score = part2(&game);
    println!("Part 2: {}", score);
    Ok(())
}

fn part1(input: &[Vec<i64>]) -> i64 {
    input.iter().map(|l| next_number(l)).sum()
}

fn next_number(input: &[i64]) -> i64 {
    let mut past = vec![*input.last().unwrap()];
    let mut current = input.to_owned();
    while !current.iter().all(|n| n == current.first().unwrap()) {
        let new = current
            .iter()
            .zip(current.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect::<Vec<i64>>();

        // println!("{:?} => {:?}", current, new);
        past.push(*new.last().unwrap());
        current = new;
    }

    past.iter().sum()
}

fn part2(input: &[Vec<i64>]) -> i64 {
    input.iter().map(|l| previous_number(l)).sum()
}

fn previous_number(input: &[i64]) -> i64 {
    let mut past = vec![*input.first().unwrap()];
    let mut current = input.to_owned();
    while !current.iter().all(|n| n == current.first().unwrap()) {
        let new = current
            .iter()
            .zip(current.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect::<Vec<i64>>();

        // println!("{:?} => {:?}", current, new);
        past.push(*new.first().unwrap());
        current = new;
    }

    past.reverse();

    let mut prev = 0;
    for i in past {
        prev = i - prev;
    }

    prev
}
