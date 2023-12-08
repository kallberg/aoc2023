use crate::day_7::HandType::{
    FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair,
};
use crate::day_7::ParseCardError::NoSuchCard;
use crate::Solver;
use anyhow::{Error, Result};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Copy)]
pub enum CamelCard {
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    T,
    J,
    Q,
    K,
    A,
}

impl Into<usize> for &CamelCard {
    fn into(self) -> usize {
        match self {
            CamelCard::C2 => 0,
            CamelCard::C3 => 1,
            CamelCard::C4 => 2,
            CamelCard::C5 => 3,
            CamelCard::C6 => 4,
            CamelCard::C7 => 5,
            CamelCard::C8 => 6,
            CamelCard::C9 => 7,
            CamelCard::T => 8,
            CamelCard::J => 9,
            CamelCard::Q => 10,
            CamelCard::K => 11,
            CamelCard::A => 12,
        }
    }
}

#[derive(Error, Debug)]
pub enum ParseCardError {
    #[error("tried to parse non card character")]
    NoSuchCard,
}

impl TryFrom<char> for CamelCard {
    type Error = ParseCardError;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            'A' => CamelCard::A,
            'K' => CamelCard::K,
            'Q' => CamelCard::Q,
            'J' => CamelCard::J,
            'T' => CamelCard::T,
            '9' => CamelCard::C9,
            '8' => CamelCard::C8,
            '7' => CamelCard::C7,
            '6' => CamelCard::C6,
            '5' => CamelCard::C5,
            '4' => CamelCard::C4,
            '3' => CamelCard::C3,
            '2' => CamelCard::C2,
            _ => return Err(NoSuchCard),
        })
    }
}

#[derive(Eq, PartialEq)]
struct Hand {
    cards: [CamelCard; 5],
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for card in &self.cards {
            write!(
                f,
                "{}",
                match card {
                    CamelCard::C2 => '2',
                    CamelCard::C3 => '3',
                    CamelCard::C4 => '4',
                    CamelCard::C5 => '5',
                    CamelCard::C6 => '6',
                    CamelCard::C7 => '7',
                    CamelCard::C8 => '8',
                    CamelCard::C9 => '9',
                    CamelCard::T => 'T',
                    CamelCard::J => 'J',
                    CamelCard::Q => 'Q',
                    CamelCard::K => 'K',
                    CamelCard::A => 'A',
                }
            )?;
        }

        Ok(())
    }
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut kinds = [0; 13];

        for card in &self.cards {
            let index: usize = card.into();
            kinds[index] += 1;
        }

        kinds.sort();
        kinds.reverse();

        match (kinds[0], kinds[1]) {
            (5, _) => FiveOfAKind,
            (4, _) => FourOfAKind,
            (3, 2) => FullHouse,
            (3, _) => ThreeOfAKind,
            (2, 2) => TwoPair,
            (2, _) => OnePair,
            _ => HighCard,
        }
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_type = self.hand_type();
        let other_type = other.hand_type();

        if self_type != other_type {
            return self_type.partial_cmp(&other_type);
        }

        self.cards.partial_cmp(&other.cards)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_type = self.hand_type();
        let other_type = other.hand_type();

        if self_type != other_type {
            return self_type.cmp(&other_type);
        }

        self.cards.cmp(&other.cards)
    }
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Debug)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Default)]
pub struct Day {
    input: String,
    plays: Vec<(Hand, u32)>,
}

impl Solver for Day {
    fn setup(&mut self, input: &str) {
        self.input = input.to_string();
    }

    fn parse(&mut self) -> Result<()> {
        for line in self.input.lines() {
            let (left, right) = line
                .split_once(' ')
                .ok_or(Error::msg("split hand and bid by whitespace"))?;

            let mut cards = [CamelCard::A; 5];

            for (index, c) in left.chars().enumerate() {
                if index >= 5 {
                    return Err(Error::msg("found more than 5 cards in a hand"));
                }

                cards[index] = CamelCard::try_from(c)?;
            }

            let bid: u32 = right.parse()?;

            self.plays.push((Hand { cards }, bid));
        }

        self.plays.sort_by(|(a, _), (b, _)| a.cmp(&b));

        Ok(())
    }

    fn part_1(&self) -> Result<String> {
        let mut sum = 0;

        for (index, (hand, bid)) in self.plays.iter().enumerate() {
            println!("{} {:?}", hand, hand.hand_type());
            sum += bid * (index as u32 + 1);
        }

        Ok(sum.to_string())
    }

    fn part_2(&self) -> Result<String> {
        Ok("Placeholder".to_string())
    }
}
