use crate::utils::{
    aocdata::{Part, TestCase},
    puzzle::{PuzzleFns, SolvePuzzle},
};
use anyhow::Result;
use itertools::Itertools;

pub struct AoC2017Day1;

impl SolvePuzzle for AoC2017Day1 {
    type Output = (Option<i32>, Option<i32>);
    fn puzzle_year_day() -> (i32, u32) {
        (2017, 1)
    }

    fn solve(input: &str) -> Result<Self::Output> {

        Ok((None, None))
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase::new(Part::A, 0, 0),
            TestCase::new(Part::B, 0, 0),
        ]
    }
}

#[test]
fn day1() -> Result<()> {
    AoC2017Day1::run_tests()?;
    let res = AoC2017Day1::try_submit()?;
    eprintln!("{res:?}");
    Ok(())
}
