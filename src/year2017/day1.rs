use crate::utils::{
    aocdata::{Part, TestCase},
    puzzle::{PuzzleFns, SolvePuzzle},
};
use anyhow::Result;
use itertools::Itertools;

pub struct AoC2017Day1;

impl SolvePuzzle for AoC2017Day1 {
    type Output = (Option<u32>, Option<u32>);
    fn puzzle_year_day() -> (i32, u32) {
        (2017, 1)
    }

    fn solve(input: &str) -> Result<Self::Output> {
        let chars = input
            .split("")
            .map(|x| x.parse::<u32>().ok())
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect_vec();
        let sum = chars
            .iter()
            .circular_tuple_windows()
            .map(|(a, b)| if a == b { *a } else { 0 })
            .sum();
        let half = chars.len() / 2;
        let sumb = (0..chars.len())
            .into_iter()
            .map(|a| (a, (a + half) % chars.len()))
            .map(|(a, b)| if chars[a] == chars[b] { chars[a] } else { 0 })
            .sum();

        Ok((Some(sum), Some(sumb)))
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase::new(Part::A, 1122, 3),
            TestCase::new(Part::A, 1111, 4),
            TestCase::new(Part::A, 1234, 0),
            TestCase::new(Part::A, 91212129, 9),
            TestCase::new(Part::B, 1212, 6),
            TestCase::new(Part::B, 1221, 0),
            TestCase::new(Part::B, 12131415, 4),
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
