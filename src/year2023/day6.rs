use crate::utils::{
    aocdata::{Part, TestCase},
    puzzle::{PuzzleFns, SolvePuzzle},
};
use anyhow::Result;
use itertools::Itertools;
use rayon::prelude::*;

pub struct AoC2023Day6;

impl SolvePuzzle for AoC2023Day6 {
    type Output = (Option<i32>, Option<i64>);
    fn puzzle_year_day() -> (i32, u32) {
        (2023, 6)
    }

    fn solve(input: &str) -> Result<Self::Output> {
        let (time_line, dist_line) = input.lines().tuples().next().unwrap();
        let times: Vec<u32> = time_line
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect_vec();
        let distances: Vec<u32> = dist_line
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect_vec();

        let part_a = times
            .iter()
            .zip(distances.iter())
            .map(|(&time, &dist)| {
                let mut wins = 0;
                for speed in 1..time {
                    let travel_time = time - speed;
                    let travel_dist = speed * travel_time;
                    if travel_dist > dist {
                        wins += 1;
                    }
                }
                wins
            })
            .reduce(|acc, elem| acc * elem);

        let time: u64 = time_line
            .split_whitespace()
            .skip(1)
            .join("")
            .parse()
            .unwrap();
        let dist: u64 = dist_line
            .split_whitespace()
            .skip(1)
            .join("")
            .parse()
            .unwrap();

        let mut wins = 0i64;
        for speed in 1..time {
            let travel_time = time - speed;
            let travel_dist = speed * travel_time;
            if travel_dist > dist {
                wins += 1;
            }
        }

        Ok((part_a, Some(wins)))
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase::new(
                Part::A,
                "Time:      7  15   30
Distance:  9  40  200",
                288,
            ),
            TestCase::new(
                Part::B,
                "Time:      7  15   30
Distance:  9  40  200",
                71503,
            ),
        ]
    }
}

#[test]
fn run() -> Result<()> {
    AoC2023Day6::run_tests()?;
    let res = AoC2023Day6::try_submit()?;
    eprintln!("{res:?}");
    Ok(())
}
