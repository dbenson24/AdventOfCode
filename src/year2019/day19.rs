use std::collections::VecDeque;

use crate::utils::{
    aocdata::{Part, TestCase},
    puzzle::{PuzzleFns, SolvePuzzle},
    World,
};
use anyhow::Result;
use glam::IVec2;
use hashbrown::HashMap;
use itertools::Itertools;
use rayon::prelude::*;

use super::intcode::IntcodeMachine;

pub struct AoC2019Day19;

impl SolvePuzzle for AoC2019Day19 {
    type Output = (Option<i64>, Option<i64>);
    fn puzzle_year_day() -> (i32, u32) {
        (2019, 19)
    }

    fn solve(input: &str) -> Result<Self::Output> {
        let mut numbers = IntcodeMachine::parse_str(input).unwrap();
        let in_emitter = |x, y| {
            if x < 0 || y < 0 {
                return 0;
            }
            let (input, output) = IntcodeMachine::spawn(numbers.clone());
            input.send(Some(x));
            input.send(Some(y));
            output.recv().unwrap().unwrap()
        };
        let count = (0..50i64)
            .flat_map(|x| (0..50i64).into_iter().map(move |y| (x, y)))
            .map(|(x, y)| in_emitter(x, y))
            .sum();

        let d = 99;
        let mut buffer = VecDeque::new();
        let mut res = (0, 0);
        buffer.push_back((3, 5));
        while let Some((x, y)) = buffer.pop_front() {
            if in_emitter(x, y) == 0 {
                dbg!(x, y);
                continue;
            }
            if in_emitter(x + d, y - d) == 1 {
                res = (x, y);
                dbg!(res);
                break;
            }

            if in_emitter(x, y + 1) == 1 {
                dbg!((x, y + 1));
                buffer.push_back((x, y + 1));
            } else if in_emitter(x + 1, y + 1) == 1 {
                dbg!((x + 1, y + 1));
                buffer.push_back((x + 1, y + 1));
            } else if in_emitter(x + 1, y) == 1 {
                dbg!((x + 1, y));
                buffer.push_back((x + 1, y));
            }
        }
        let (x, y) = res;
        dbg!(x, y);
        dbg!(x, y - d);

        let mut world: World<char> = World {
            world: HashMap::new(),
        };

        for (x, y) in (0..50i64).flat_map(|x| (0..50i64).into_iter().map(move |y| (x, y))) {
            let c = if in_emitter(x, y) == 1 { '#' } else { '.' };
            world.world.insert(IVec2::new(x as i32, y as i32), c);
        }
        world.pretty_print(false);

        // let (input, output) = IntcodeMachine::spawn(numbers.clone());
        // input.send(Some(0));
        // input.send(Some(0));
        // dbg!(output.recv());
        // input.send(Some(0));
        // input.send(Some(1));
        // dbg!(output.recv());
        let val = (x * 10000) + (y - d);
        Ok((Some(count), Some(val)))
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            // TestCase::new(Part::A, 0, 0),
            // TestCase::new(Part::B, 0, 0),
        ]
    }
}

#[test]
fn day1() -> Result<()> {
    AoC2019Day19::run_tests()?;
    let res = AoC2019Day19::try_submit()?;
    eprintln!("{res:?}");
    Ok(())
}
