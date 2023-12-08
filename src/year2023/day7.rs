use std::{cmp::Ordering, collections::HashMap};

use crate::utils::{
    aocdata::{Part, TestCase},
    puzzle::{PuzzleFns, SolvePuzzle},
};
use anyhow::Result;
use itertools::Itertools;
use rayon::prelude::*;

pub struct AoC2023Day7;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Card {
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    CT,
    CJ,
    CQ,
    CK,
    CA,
}

impl TryFrom<char> for Card {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::prelude::v1::Result<Self, Self::Error> {
        match value {
            '2' => Ok(Card::C2),
            '3' => Ok(Card::C3),
            '4' => Ok(Card::C4),
            '5' => Ok(Card::C5),
            '6' => Ok(Card::C6),
            '7' => Ok(Card::C7),
            '8' => Ok(Card::C8),
            '9' => Ok(Card::C9),
            'T' => Ok(Card::CT),
            'J' => Ok(Card::CJ),
            'Q' => Ok(Card::CQ),
            'K' => Ok(Card::CK),
            'A' => Ok(Card::CA),
            _ => Err(anyhow::anyhow!("No matching card {}", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CardJ {
    CJ,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    CT,
    CQ,
    CK,
    CA,
}

impl TryFrom<char> for CardJ {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::prelude::v1::Result<Self, Self::Error> {
        match value {
            '2' => Ok(CardJ::C2),
            '3' => Ok(CardJ::C3),
            '4' => Ok(CardJ::C4),
            '5' => Ok(CardJ::C5),
            '6' => Ok(CardJ::C6),
            '7' => Ok(CardJ::C7),
            '8' => Ok(CardJ::C8),
            '9' => Ok(CardJ::C9),
            'T' => Ok(CardJ::CT),
            'J' => Ok(CardJ::CJ),
            'Q' => Ok(CardJ::CQ),
            'K' => Ok(CardJ::CK),
            'A' => Ok(CardJ::CA),
            _ => Err(anyhow::anyhow!("No matching card {}", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub hand_type: HandType,
}

impl Hand {
    pub fn from_cards(cards: Vec<Card>) -> Self {
        let card_counts = cards.iter().counts();
        let match_counts = card_counts.into_values().counts();
        // dbg!(&match_counts);
        let hand_type = get_hand_type(&match_counts);
        Self { cards, hand_type }
    }
}

fn get_hand_type(match_counts: &HashMap<usize, usize>) -> HandType {
    if *match_counts.get(&5).unwrap_or(&0) == 1 {
        HandType::FiveOfAKind
    } else if *match_counts.get(&4).unwrap_or(&0) == 1 {
        HandType::FourOfAKind
    } else if *match_counts.get(&3).unwrap_or(&0) == 1
        && *match_counts.get(&2).unwrap_or(&0) == 1
    {
        HandType::FullHouse
    } else if *match_counts.get(&3).unwrap_or(&0) == 1 {
        HandType::ThreeOfAKind
    } else if *match_counts.get(&2).unwrap_or(&0) == 2 {
        HandType::TwoPair
    } else if *match_counts.get(&2).unwrap_or(&0) == 1 {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let type_cmp = self.hand_type.cmp(&other.hand_type);
        if type_cmp == Ordering::Equal {
            for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                let card_cmp = self_card.cmp(other_card);
                if card_cmp != Ordering::Equal {
                    return card_cmp;
                }
            }
        } else {
            return type_cmp;
        }

        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HandJ {
    pub cards: Vec<CardJ>,
    pub hand_type: HandType,
}

impl HandJ {
    pub fn from_cards(cards: Vec<CardJ>) -> Self {
        let mut card_counts = cards.iter().counts();
        let joker_counts = card_counts.remove(&CardJ::CJ).unwrap_or(0);
        // let mut match_counts: std::collections::HashMap<usize, usize> = card_counts.clone().into_values().map(|x| x + joker_counts).counts();
        let mut unmodified_counts = card_counts.into_values().counts();
        // dbg!(&match_counts);
        // let hand_type = if *match_counts.get(&5).unwrap_or(&0) >= 1 {
        //     HandType::FiveOfAKind
        // } else if *match_counts.get(&4).unwrap_or(&0) >= 1 {
        //     HandType::FourOfAKind
        // } else if (joker_counts == 1
        //     && *unmodified_counts.get(&2).unwrap_or(&0) == 2)
        //     || (joker_counts == 0
        //         && *match_counts.get(&3).unwrap_or(&0) == 1
        //         && *match_counts.get(&2).unwrap_or(&0) == 1)
        // {
        //     HandType::FullHouse
        // } else if *match_counts.get(&3).unwrap_or(&0) >= 1 {
        //     HandType::ThreeOfAKind
        // } else if *match_counts.get(&2).unwrap_or(&0) >= 2 {
        //     HandType::TwoPair
        // } else if *match_counts.get(&2).unwrap_or(&0) >= 1 {
        //     HandType::OnePair
        // } else {
        //     HandType::HighCard
        // };
        // let hand_type = if joker_counts == 5 {
        //     HandType::FiveOfAKind
        // } else {
        //     hand_type
        // };
        let hand_type = get_hand_type(&unmodified_counts);
        let hand_type = match joker_counts {
            5 => HandType::FiveOfAKind,
            4 => HandType::FiveOfAKind,
            3 => match hand_type {
                HandType::OnePair => HandType::FiveOfAKind,
                _ => HandType::FourOfAKind,
            },
            2 => match hand_type {
                HandType::ThreeOfAKind => HandType::FiveOfAKind,
                HandType::OnePair => HandType::FourOfAKind,
                _ => HandType::ThreeOfAKind
            }
            1 => match hand_type {
                HandType::FourOfAKind => HandType::FiveOfAKind,
                HandType::ThreeOfAKind => HandType::FourOfAKind,
                HandType::TwoPair => HandType::FullHouse,
                HandType::OnePair => HandType::ThreeOfAKind,
                _ => HandType::OnePair
            }
            _ => hand_type
        };

        Self { cards, hand_type }
    }
}

impl Ord for HandJ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let type_cmp = self.hand_type.cmp(&other.hand_type);
        if type_cmp == Ordering::Equal {
            for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                let card_cmp = self_card.cmp(other_card);
                if card_cmp != Ordering::Equal {
                    return card_cmp;
                }
            }
        } else {
            return type_cmp;
        }

        Ordering::Equal
    }
}

impl PartialOrd for HandJ {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl SolvePuzzle for AoC2023Day7 {
    type Output = (Option<i32>, Option<i32>);
    fn puzzle_year_day() -> (i32, u32) {
        (2023, 7)
    }

    fn solve(input: &str) -> Result<Self::Output> {
        let mut hands = input
            .lines()
            .map(|line| {
                let (hand, bid) = line.split_once(" ").unwrap();
                let cards: Vec<Card> = hand.chars().map(|c| c.try_into().unwrap()).collect();
                (Hand::from_cards(cards), bid.parse::<i32>().unwrap())
            })
            .collect_vec();
        hands.sort_by_key(|k| k.0.clone());
        // dbg!(&hands);
        let part_a: i32 = hands
            .iter()
            .enumerate()
            .map(|(rank, (_, bid))| (rank as i32 + 1) * *bid)
            .sum();

        let mut hands_j = input
            .lines()
            .map(|line| {
                let (hand, bid) = line.split_once(" ").unwrap();
                let cards: Vec<CardJ> = hand.chars().map(|c| c.try_into().unwrap()).collect();
                (HandJ::from_cards(cards), bid.parse::<i32>().unwrap())
            })
            .collect_vec();
        hands_j.sort_by_key(|k| k.0.clone());
        let part_b: i32 = hands_j
            .iter()
            .enumerate()
            .map(|(rank, (_, bid))| (rank as i32 + 1) * *bid)
            .sum();
        dbg!(hands_j
            .iter()
            .filter(|x| x.0.cards.contains(&CardJ::CJ) && x.0.hand_type == HandType::FullHouse)
            .collect_vec());
        Ok((Some(part_a), Some(part_b)))
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase::new(
                Part::A,
                "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
                6440,
            ),
            TestCase::new(
                Part::B,
                "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
                5905,
            ),
            // TestCase::new(Part::B, 0, 0),
        ]
    }
}

#[test]
fn run() -> Result<()> {
    AoC2023Day7::run_tests()?;
    let res = AoC2023Day7::try_submit()?;
    eprintln!("{res:?}");
    Ok(())
}
