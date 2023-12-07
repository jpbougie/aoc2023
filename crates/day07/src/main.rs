use core::panic;
use std::{collections::HashMap, str::FromStr};

fn main() {
    solve(include_str!("../input.txt")).unwrap();
}

fn solve(input: &str) -> Result<(), anyhow::Error> {
    let mut game = parse(input)?;

    game.sort();

    let score = game
        .iter()
        .enumerate()
        .map(|(i, play)| (i as u32 + 1) * play.bid)
        .sum::<u32>();

    println!("Part 1: {}", score);

    let mut joker_game = parse_joker(input)?;
    joker_game.sort();
    let score = joker_game
        .iter()
        .enumerate()
        .map(|(i, play)| (i as u32 + 1) * play.bid)
        .sum::<u32>();
    println!("Part 2: {}", score);

    Ok(())
}

fn parse(input: &str) -> Result<Vec<Play>, anyhow::Error> {
    input
        .lines()
        .map(|l| l.parse::<Play>())
        .collect::<Result<Vec<Play>, anyhow::Error>>()
}

fn parse_joker(input: &str) -> Result<Vec<JokerPlay>, anyhow::Error> {
    input
        .lines()
        .map(|l| l.parse::<JokerPlay>())
        .collect::<Result<Vec<JokerPlay>, anyhow::Error>>()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Play {
    hand: Hand,
    bid: u32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    High,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl FromStr for Play {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(' ').unwrap();

        Ok(Self {
            hand: hand.parse()?,
            bid: bid.parse().unwrap(),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.kind()
            .cmp(&other.kind())
            .then_with(|| self.cards.cmp(&other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn kind(&self) -> Type {
        let mut freq = HashMap::new();
        for card in self.cards.iter() {
            let fr = freq.entry(card).or_default();
            *fr += 1;
        }

        match freq.len() {
            1 => Type::FiveOfAKind,
            2 => match freq.values().next() {
                Some(1) | Some(4) => Type::FourOfAKind,
                _ => Type::FullHouse,
            },
            3 => {
                if freq.values().any(|v| *v == 3) {
                    Type::ThreeOfAKind
                } else {
                    Type::TwoPair
                }
            }
            4 => Type::OnePair,
            5 => Type::High,
            _ => panic!("Unexpected variety of cards"),
        }
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s
            .split("")
            .filter_map(|s| {
                if s.is_empty() {
                    None
                } else {
                    Some(s.parse::<Card>())
                }
            })
            .collect::<Result<Vec<Card>, anyhow::Error>>()?;

        Ok(Self {
            cards: cards.try_into().unwrap(),
        })
    }
}

impl From<[JokerCard; 5]> for Hand {
    fn from(value: [JokerCard; 5]) -> Self {
        let cards = [
            value[0].into(),
            value[1].into(),
            value[2].into(),
            value[3].into(),
            value[4].into(),
        ];

        Self { cards }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<JokerCard> for Card {
    fn from(value: JokerCard) -> Self {
        match value {
            JokerCard::Joker => Self::Jack,
            JokerCard::Two => Self::Two,
            JokerCard::Three => Self::Three,
            JokerCard::Four => Self::Four,
            JokerCard::Five => Self::Five,
            JokerCard::Six => Self::Six,
            JokerCard::Seven => Self::Seven,
            JokerCard::Eight => Self::Eight,
            JokerCard::Nine => Self::Nine,
            JokerCard::Ten => Self::Ten,
            JokerCard::Queen => Self::Queen,
            JokerCard::King => Self::King,
            JokerCard::Ace => Self::Ace,
        }
    }
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('2') => Ok(Self::Two),
            Some('3') => Ok(Self::Three),
            Some('4') => Ok(Self::Four),
            Some('5') => Ok(Self::Five),
            Some('6') => Ok(Self::Six),
            Some('7') => Ok(Self::Seven),
            Some('8') => Ok(Self::Eight),
            Some('9') => Ok(Self::Nine),
            Some('T') => Ok(Self::Ten),
            Some('J') => Ok(Self::Jack),
            Some('Q') => Ok(Self::Queen),
            Some('K') => Ok(Self::King),
            Some('A') => Ok(Self::Ace),
            x => Err(anyhow::anyhow!("Unknown card: {:?}", x)),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
enum JokerCard {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl FromStr for JokerCard {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('2') => Ok(Self::Two),
            Some('3') => Ok(Self::Three),
            Some('4') => Ok(Self::Four),
            Some('5') => Ok(Self::Five),
            Some('6') => Ok(Self::Six),
            Some('7') => Ok(Self::Seven),
            Some('8') => Ok(Self::Eight),
            Some('9') => Ok(Self::Nine),
            Some('T') => Ok(Self::Ten),
            Some('J') => Ok(Self::Joker),
            Some('Q') => Ok(Self::Queen),
            Some('K') => Ok(Self::King),
            Some('A') => Ok(Self::Ace),
            x => Err(anyhow::anyhow!("Unknown card: {:?}", x)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct JokerHand {
    cards: [JokerCard; 5],
    kind: Type,
}

impl JokerHand {
    fn new(cards: [JokerCard; 5]) -> Self {
        let kind = JokerHand::compute_kind(&cards);
        Self { cards, kind }
    }

    fn compute_kind(cards: &[JokerCard; 5]) -> Type {
        let cards_without_joker = cards
            .iter()
            .filter(|c| **c != JokerCard::Joker)
            .collect::<Vec<_>>();

        let mut freqs_without_jokers: HashMap<JokerCard, usize> = HashMap::new();
        let jokers = 5 - cards_without_joker.len();
        for c in cards_without_joker {
            let ent = freqs_without_jokers.entry(*c).or_default();
            *ent += 1usize;
        }

        match jokers {
            4 | 5 => Type::FiveOfAKind,
            3 => match freqs_without_jokers.len() {
                1 => Type::FiveOfAKind, // AAJJJ
                2 => Type::FourOfAKind, // KAJJJ
                _ => panic!(),
            },
            2 => match freqs_without_jokers.len() {
                1 => Type::FiveOfAKind,  // AAAJJ
                2 => Type::FourOfAKind,  // AAKJJ
                3 => Type::ThreeOfAKind, // AKTJJ
                _ => panic!(),
            },
            1 => match freqs_without_jokers.len() {
                1 => Type::FiveOfAKind, // AAAAJ
                2 => {
                    if *freqs_without_jokers.values().next().unwrap() == 2 {
                        Type::FullHouse // AAKKJJ
                    } else {
                        Type::FourOfAKind // AAAKJ
                    }
                }
                3 => Type::ThreeOfAKind, // AAKTJ
                4 => Type::OnePair,      //
                _ => panic!(),
            },
            0 => Hand::from(*cards).kind(),
            _ => panic!(),
        }
    }
}

impl Ord for JokerHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.kind
            .cmp(&other.kind)
            .then_with(|| self.cards.cmp(&other.cards))
    }
}

impl PartialOrd for JokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for JokerHand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s
            .split("")
            .filter_map(|s| {
                if s.is_empty() {
                    None
                } else {
                    Some(s.parse::<JokerCard>())
                }
            })
            .collect::<Result<Vec<JokerCard>, anyhow::Error>>()?;

        Ok(Self::new(cards.try_into().unwrap()))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct JokerPlay {
    hand: JokerHand,
    bid: u32,
}

impl FromStr for JokerPlay {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(' ').unwrap();

        Ok(Self {
            hand: hand.parse()?,
            bid: bid.parse().unwrap(),
        })
    }
}
