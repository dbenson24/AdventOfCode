use core::fmt::Debug;
use std::{
    collections::BTreeMap,
    convert::{TryFrom, TryInto},
    fmt::Display,
};

use super::{
    aocapi::{get_input, read_session_cookie},
    aocdata::{AnswerState, Part, PuzzleState, TestCase},
    PuzzleAnswer,
};
use anyhow::{anyhow, Context, Error, Result};

/***
 * Goals: Make it easier to to AoC from Rust
 * Automatically download and cache inputs
 * Know how to run a solution using the input
 * Able to add test cases
 * Run submission
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PuzzleYear(i32);
impl PuzzleYear {
    pub fn get(&self) -> i32 {
        self.0
    }
}
const FIRST_EVENT_YEAR: PuzzleYear = PuzzleYear(2015);
impl TryFrom<i32> for PuzzleYear {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 2015 {
            Err(anyhow!("Invalid year, First puzzle was in 2015."))
        } else {
            Ok(PuzzleYear(value))
        }
    }
}

impl Display for PuzzleYear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PuzzleDay(u32);
impl PuzzleDay {
    pub fn get(&self) -> u32 {
        self.0
    }
}
const FIRST_PUZZLE_DAY: PuzzleDay = PuzzleDay(1);
const LAST_PUZZLE_DAY: PuzzleDay = PuzzleDay(25);
impl TryFrom<u32> for PuzzleDay {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value > 25 {
            Err(anyhow!("Invalid day, there are only 25 days."))
        } else {
            Ok(PuzzleDay(value))
        }
    }
}

impl Display for PuzzleDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Clone)]
pub struct PuzzleInfo {
    pub id: (PuzzleYear, PuzzleDay),
    pub solver: &'static dyn Fn(&str) -> Result<PuzzleAnswer>,
    pub tests: Vec<TestCase>,
}

pub trait SolvePuzzle {
    type Output: Into<PuzzleAnswer>;
    fn puzzle_year_day() -> (i32, u32);
    fn solve(input: &str) -> Result<Self::Output>;
    fn test_cases() -> Vec<TestCase> {
        Vec::new()
    }
}

pub trait PuzzleFns {
    fn run_tests() -> Result<()>;
    fn puzzle_info() -> PuzzleInfo;
    fn try_submit() -> Result<(bool, bool)>;
}

impl<T> PuzzleFns for T
where
    T: Sized + SolvePuzzle + 'static,
{
    fn run_tests() -> Result<()>
    where
        Self: Sized + 'static,
    {
        Self::puzzle_info().run_tests()
    }
    fn puzzle_info() -> PuzzleInfo
    where
        Self: Sized + 'static,
    {
        PuzzleInfo::new::<Self>()
    }
    fn try_submit() -> Result<(bool, bool)>
    where
        Self: Sized + 'static,
    {
        Self::puzzle_info().try_submit()
    }
}

fn run_solve<T: SolvePuzzle + 'static>(input: &str) -> Result<PuzzleAnswer> {
    let res = T::solve(input)?;
    Ok(res.into())
}

impl PuzzleInfo {
    pub fn new<T: SolvePuzzle + 'static>() -> PuzzleInfo {
        let (raw_year, raw_day) = T::puzzle_year_day();
        let year = PuzzleYear::try_from(raw_year)
            .context("Year should be valid")
            .unwrap();
        let day = PuzzleDay::try_from(raw_day)
            .context("Day should be valid")
            .unwrap();

        PuzzleInfo {
            id: (year, day),
            solver: (&run_solve::<T>) as &'static dyn Fn(&str) -> Result<PuzzleAnswer>,
            tests: T::test_cases(),
        }
    }

    pub fn run_tests(&self) -> Result<()> {
        for test in &self.tests {
            let result = (self.solver)(&test.input.get_value()?)?;
            let answer = match test.part {
                Part::A => result.0,
                Part::B => result.1,
            };
            let answer = answer.ok_or(anyhow!("Expected the solution to return an answer."))?;
            let expected = test.expected.get_value()?;
            #[cfg(test)]
            {
                assert_eq!(answer, expected);
            }
            if answer != expected {
                return Err(anyhow!("Expected: {answer} to equal: {expected}"));
            }
        }
        Ok(())
    }

    pub fn try_submit(&self) -> Result<(bool, bool)> {
        let cookie = read_session_cookie()?;
        let (year, day) = self.id;
        let input = get_input(&cookie, year, day)?;
        let answer = (self.solver)(&input)?;
        let mut state = PuzzleState::load(year, day);
        let result = state.try_submit(&cookie, year, day, &answer)?;
        state.save(year, day);
        Ok(result)
    }
}

#[derive(Clone)]
pub struct AoC {
    puzzles: BTreeMap<(PuzzleYear, PuzzleDay), PuzzleInfo>,
}

impl AoC {
    pub fn new() -> Self {
        AoC {
            puzzles: BTreeMap::new(),
        }
    }

    pub fn register<T: SolvePuzzle + 'static>(mut self) -> Self {
        let info = PuzzleInfo::new::<T>();
        if let Some(puzzle) = self.puzzles.insert(info.id.clone(), info) {
            eprintln!(
                "There was already a puzzle registered for {} day {}",
                puzzle.id.0, puzzle.id.1
            );
        }
        self
    }

    pub fn run(&self) {
        let ids: Vec<_> = self.puzzles.keys().collect();
        dbg!(ids);
    }

    pub fn test_puzzle(&self, year: PuzzleYear, day: PuzzleDay) -> Result<()> {
        let puzzle = self
            .puzzles
            .get(&(year, day))
            .ok_or(anyhow!("No puzzle for {year} day {day}"))?;
        puzzle.run_tests()
    }
}

pub struct AoC2022Day1PartA;

impl SolvePuzzle for AoC2022Day1PartA {
    type Output = &'static str;
    fn puzzle_year_day() -> (i32, u32) {
        (2022, 1)
    }

    fn solve(input: &str) -> Result<Self::Output> {
        Ok("1")
    }

    fn test_cases() -> Vec<TestCase> {
        vec![TestCase::new(Part::A, 1, 0)]
    }
}

#[test]
fn day1() {
    AoC2022Day1PartA::run_tests().expect("to finish");
}

pub struct AoC2022Day1PartB;
impl SolvePuzzle for AoC2022Day1PartB {
    type Output = PuzzleAnswer;
    fn puzzle_year_day() -> (i32, u32) {
        (2022, 1)
    }

    fn solve(input: &str) -> Result<Self::Output> {
        Ok(PuzzleAnswer(Some("1".to_owned()), None))
    }
}

pub struct AoC2022Day2;
impl SolvePuzzle for AoC2022Day2 {
    type Output = (Option<String>, String);
    fn puzzle_year_day() -> (i32, u32) {
        (2022, 2)
    }

    fn solve(input: &str) -> Result<Self::Output> {
        Ok((Some("1".to_owned()), "2".to_owned()))
    }
}

pub struct AoC2022Day3;
impl SolvePuzzle for AoC2022Day3 {
    type Output = (Option<String>, String);
    fn puzzle_year_day() -> (i32, u32) {
        (2022, 3)
    }

    fn solve(input: &str) -> Result<Self::Output> {
        Ok((Some("1".to_owned()), "2".to_owned()))
    }
}

#[test]
pub fn basic_test() {
    let aoc = AoC::new()
        .register::<AoC2022Day1PartA>()
        .register::<AoC2022Day1PartB>()
        .register::<AoC2022Day2>()
        .register::<AoC2022Day3>();
    aoc.run();
}
