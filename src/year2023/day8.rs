use std::iter::Repeat;

use crate::utils::{
    aocdata::{Part, TestCase},
    puzzle::{PuzzleFns, SolvePuzzle},
};
use anyhow::Result;
use hashbrown::HashMap;
use itertools::Itertools;
use rayon::prelude::*;

pub struct AoC2023Day8;

fn find_cycle(map: &HashMap<&str, (&str, &str)>, moves: &[char], start: &str) -> (usize, usize) {
    let mut position = start;
    let mut curr_move = 0;
    while !position.ends_with("Z") {
        let (left, right) = map[position];
        position = if moves[curr_move % moves.len()] == 'R' {
            right
        } else {
            left
        };
        curr_move += 1;
    }
    let cycle_start = curr_move;
    let (left, right) = map[position];
    position = if moves[curr_move % moves.len()] == 'R' {
        right
    } else {
        left
    };
    curr_move += 1;
    while !position.ends_with("Z") {
        let (left, right) = map[position];
        position = if moves[curr_move % moves.len()] == 'R' {
            right
        } else {
            left
        };
        curr_move += 1;
    }
    (cycle_start, curr_move - cycle_start)
}

impl SolvePuzzle for AoC2023Day8 {
    type Output = (Option<i32>, Option<usize>);
    fn puzzle_year_day() -> (i32, u32) {
        (2023, 8)
    }

    fn solve(input: &str) -> Result<Self::Output> {
        let mut lines = input.lines();
        let moves = lines.next().unwrap().chars().collect_vec();
        lines.next().unwrap();
        let map: HashMap<_, _> = lines
            .map(|line| {
                let (key, rest) = line.split_once(" = ").unwrap();
                let rest = rest.strip_prefix("(").unwrap();
                let rest = rest.strip_suffix(")").unwrap();
                let (left, right) = rest.split_once(", ").unwrap();
                (key, (left, right))
            })
            .collect();
        let mut position = "AAA";
        let mut curr_move = 0;
        if map.contains_key("AAA") {
            while position != "ZZZ" {
                let (left, right) = map[position];
                position = if moves[curr_move % moves.len()] == 'R' {
                    right
                } else {
                    left
                };
                curr_move += 1;
            }
        }

        let mut positions = map
            .keys()
            .filter(|s| s.ends_with("A"))
            .map(|s| *s)
            .collect_vec();
        let mut cycles = positions
            .iter()
            .map(|s| find_cycle(&map, &moves, s))
            .collect_vec();
        let b = cycles
            .iter()
            .map(|x| x.0)
            .reduce(|acc, x| num::integer::lcm(acc, x))
            .unwrap();

        Ok((Some(curr_move as i32), Some(b)))
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase::new(
                Part::A,
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
                6,
            ),
            TestCase::new(
                Part::B,
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
                6,
            ),
        ]
    }
}

#[test]
fn run() -> Result<()> {
    AoC2023Day8::run_tests()?;
    let res = AoC2023Day8::try_submit()?;
    eprintln!("{res:?}");
    Ok(())
}
