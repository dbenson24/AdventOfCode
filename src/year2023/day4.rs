use std::str::FromStr;

use crate::utils::{
    aocdata::{Part, TestCase},
    puzzle::{PuzzleFns, SolvePuzzle},
};
use anyhow::Result;
use hashbrown::HashSet;
use itertools::Itertools;

pub struct AoC2023Day4;

#[derive(Debug, Clone)]
pub struct Game {
    pub id: u32,
    pub winning: HashSet<u32>,
    pub hand: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct GameResult {
    pub matches: u32,
    pub copies: u32,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, rounds) = s.split_once(": ").unwrap();
        let id = id.split_whitespace().nth(1).unwrap();
        let (winning, hand) = rounds.split_once(" | ").unwrap();
        dbg!(winning, hand);
        Ok(Self {
            id: id.parse()?,
            winning: winning
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
            hand: hand
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect_vec(),
        })
    }
}

impl SolvePuzzle for AoC2023Day4 {
    type Output = (Option<u32>, Option<u32>);
    fn puzzle_year_day() -> (i32, u32) {
        (2023, 4)
    }

    fn solve(input: &str) -> Result<Self::Output> {
        let games: Vec<Game> = input
            .lines()
            .map(|line| line.parse().unwrap())
            .collect_vec();
        let part_a: u32 = games
            .iter()
            .map(|game| {
                let counts = game.hand.iter().counts();
                let sum: u32 = counts
                    .iter()
                    .filter(|(key, _val)| game.winning.contains(key))
                    .map(|(_key, x)| *x as u32)
                    .sum();
                if sum > 0 {
                    2u32.pow(sum - 1)
                } else {
                    0
                }
            })
            .sum();
        let mut results: Vec<GameResult> = games
            .iter()
            .map(|game| {
                let counts = game.hand.iter().counts();
                let sum: u32 = counts
                    .iter()
                    .filter(|(key, _val)| game.winning.contains(key))
                    .map(|(_key, x)| *x as u32)
                    .sum();
                GameResult {
                    matches: sum,
                    copies: 1,
                }
            })
            .collect();
        for i in 0..results.len() {
            for j in 1..=results[i].matches as usize {
                if i + j < results.len() && results[i + j].copies > 0 {
                    results[i + j].copies = results[i + j].copies + results[i].copies
                }
            }
        }
        let part_b = results.iter().map(|r| r.copies).sum();

        Ok((Some(part_a), Some(part_b)))
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase::new(
                Part::A,
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
                13,
            ),
            TestCase::new(
                Part::B,
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
                30,
            ),
        ]
    }
}

#[test]
fn day() -> Result<()> {
    AoC2023Day4::run_tests()?;
    let res = AoC2023Day4::try_submit()?;
    eprintln!("{res:?}");
    Ok(())
}
