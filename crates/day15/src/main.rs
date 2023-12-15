use std::str::FromStr;

use anyhow::Result;
fn main() {
    solve(include_str!("../input.txt")).unwrap();
}

fn solve(input: &str) -> Result<()> {
    let part1 = input.split(',').map(hash).sum::<u64>();
    println!("{}", part1);

    println!("{}", part2(input)?);

    Ok(())
}

fn part2(input: &str) -> Result<usize> {
    let mut boxes = Vec::with_capacity(256);
    for _ in 0..256 {
        boxes.push(Box::default());
    }

    let instructions: Vec<Instruction> = input
        .split(',')
        .map(FromStr::from_str)
        .collect::<Result<Vec<_>>>()?;

    for instruction in instructions {
        match instruction {
            Instruction::Remove(label) => {
                let hash = hash(&label);
                boxes
                    .get_mut(hash as usize)
                    .unwrap()
                    .lenses
                    .retain(|l| l.label != label);
            }
            Instruction::Assign(label, length) => {
                let hash = hash(&label);
                let b = boxes.get_mut(hash as usize).unwrap();

                let mut found = false;
                for lens in b.lenses.iter_mut() {
                    if lens.label == label {
                        lens.length = length;
                        found = true;
                        break;
                    }
                }

                if !found {
                    b.lenses.push(Lens { length, label });
                }
            }
        }
    }

    let mut power = 0;

    for (i, b) in boxes.iter().enumerate() {
        for (il, l) in b.lenses.iter().enumerate() {
            power += (i + 1) * (il + 1) * l.length;
        }
    }

    Ok(power)
}

fn hash(input: &str) -> u64 {
    input.chars().fold(0, |mut acc, ch| {
        let num: u32 = ch.into();
        acc += num as u64;
        acc *= 17;
        acc %= 256;

        acc
    })
}
#[derive(Default)]
struct Box {
    lenses: Vec<Lens>,
}

struct Lens {
    length: usize,
    label: String,
}

enum Instruction {
    Remove(String),
    Assign(String, usize),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        if s.ends_with('-') {
            Ok(Instruction::Remove(s.trim_end_matches('-').to_string()))
        } else if let Some((label, length)) = s.split_once('=') {
            Ok(Instruction::Assign(
                label.to_string(),
                length.parse::<usize>()?,
            ))
        } else {
            Err(anyhow::anyhow!("Unrecognized pattern {s}"))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hash;

    #[test]
    fn test_hash() {
        assert_eq!(52, hash("HASH"));
    }
}
