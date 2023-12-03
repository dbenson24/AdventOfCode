use std::str::FromStr;

use crate::utils::{
    aocdata::{Part, TestCase},
    puzzle::{PuzzleFns, SolvePuzzle},
};
use anyhow::Result;
use itertools::Itertools;

pub struct AoC2023Day2;

#[derive(Debug, Clone)]
pub struct Game {
    id: u32,
    rounds: Vec<Round>,
}

#[derive(Debug, Clone)]
pub struct Round {
    red: u32,
    blue: u32,
    green: u32,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, rounds) = s.split_once(": ").unwrap();
        let (_, id) = id.split_once(" ").unwrap();
        let rounds = rounds
            .split("; ")
            .map(|round| round.parse().unwrap())
            .collect_vec();
        dbg!(id);
        Ok(Self {
            id: id.parse()?,
            rounds: rounds,
        })
    }
}

impl FromStr for Round {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret_val = Self {
            red: 0,
            blue: 0,
            green: 0,
        };

        for draw in s.split(", ") {
            let (x, color) = draw.split_once(" ").unwrap();
            dbg!(x, color);
            match color {
                "red" => ret_val.red = x.parse()?,
                "green" => ret_val.green = x.parse()?,
                "blue" => ret_val.blue = x.parse()?,
                _ => (),
            }
        }

        Ok(ret_val)
    }
}

impl SolvePuzzle for AoC2023Day2 {
    type Output = (Option<u32>, Option<i32>);
    fn puzzle_year_day() -> (i32, u32) {
        (2023, 2)
    }

    fn solve(input: &str) -> Result<Self::Output> {
        let games: Vec<Game> = input
            .lines()
            .map(|line| line.parse().unwrap())
            .collect_vec();
        let part_a = games
            .iter()
            .filter(|g| {
                g.rounds
                    .iter()
                    .all(|r| r.blue <= 14 && r.red <= 12 && r.green <= 13)
            })
            .map(|g| g.id)
            .sum();

        let part_b = games
            .iter()
            .map(|game| {
                let red_max = game.rounds.iter().map(|r| r.red).max().unwrap_or(0);
                let green_max = game.rounds.iter().map(|r| r.green).max().unwrap_or(0);
                let blue_max = game.rounds.iter().map(|r| r.blue).max().unwrap_or(0);
                (red_max * green_max * blue_max) as i32
            })
            .sum();

        Ok((Some(part_a), Some(part_b)))
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase::new(
                Part::A,
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
                8,
            ),
            TestCase::new(
                Part::B,
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
                2286,
            ),
        ]
    }
}

#[test]
fn day() -> Result<()> {
    AoC2023Day2::run_tests()?;
    let res = AoC2023Day2::try_submit()?;
    eprintln!("{res:?}");
    Ok(())
}
