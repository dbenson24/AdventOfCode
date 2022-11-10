use std::{
    convert::TryFrom,
    fs::{self, read_to_string, File},
    io::Read,
};

use super::{
    aocapi::{get_puzzle_dir, submit_answer},
    puzzle::{PuzzleDay, PuzzleYear},
    puzzleanswer::PuzzleAnswer,
};
use anyhow::Result;
use html2text::from_read;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum AoCData {
    File(String),
    Raw(String),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Part {
    A,
    B,
}

impl Part {
    pub fn get_level(&self) -> &'static str {
        match self {
            Part::A => "1",
            Part::B => "2",
        }
    }
}

impl AoCData {
    pub fn get_value(&self) -> Result<String> {
        match self {
            AoCData::Raw(value) => Ok(value.clone()),
            AoCData::File(path) => {
                let value = read_to_string(path)?;
                Ok(value)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct TestCase {
    pub part: Part,
    pub input: AoCData,
    pub expected: AoCData,
}

impl TestCase {
    pub fn new(part: Part, input: impl ToString, expected: impl ToString) -> Self {
        Self {
            part,
            input: AoCData::Raw(input.to_string()),
            expected: AoCData::Raw(expected.to_string()),
        }
    }
    pub fn from_files(part: Part, input: impl ToString, expected: impl ToString) -> Self {
        Self {
            part,
            input: AoCData::File(input.to_string()),
            expected: AoCData::File(expected.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BadAnswers {
    GreaterThan(i128),
    LessThan(i128),
    Not(i128),
    Raw(String),
}

impl BadAnswers {
    pub fn str_is_bad(&self, answer: &str) -> bool {
        match self {
            BadAnswers::GreaterThan(_) => false,
            BadAnswers::LessThan(_) => false,
            BadAnswers::Not(_) => false,
            BadAnswers::Raw(prev) => answer == prev,
        }
    }
    pub fn int_is_bad(&self, answer: i128) -> bool {
        match self {
            BadAnswers::GreaterThan(prev) => answer <= *prev,
            BadAnswers::LessThan(prev) => answer >= *prev,
            BadAnswers::Not(prev) => answer == *prev,
            BadAnswers::Raw(_) => false,
        }
    }

    pub fn from_response(response: &str, answer: &str) -> Self {
        let too_high = Regex::new(r"answer is too high").unwrap();
        let too_low = Regex::new(r"answer is too low").unwrap();
        let is_num = answer.parse::<i128>();

        if let Ok(num) = is_num {
            if too_high.is_match(response) {
                BadAnswers::LessThan(num)
            } else if too_low.is_match(response) {
                BadAnswers::GreaterThan(num)
            } else {
                BadAnswers::Not(num)
            }
        } else {
            BadAnswers::Raw(answer.to_owned())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AnswerState {
    Solution(String),
    Attempts(Vec<BadAnswers>),
    PreviouslyDone,
}

impl AnswerState {
    pub fn answer_is_ok(&self, answer: &str) -> bool {
        match self {
            AnswerState::Solution(expected) => expected == answer,
            AnswerState::Attempts(prev) => {
                let num = answer.parse();
                if let Ok(num) = num {
                    !prev.iter().any(|x| x.int_is_bad(num))
                } else {
                    !prev.iter().any(|x| x.str_is_bad(answer))
                }
            }
            AnswerState::PreviouslyDone => false,
        }
    }

    pub fn is_solved(&self) -> bool {
        match self {
            AnswerState::Solution(_) => true,
            AnswerState::Attempts(_) => false,
            AnswerState::PreviouslyDone => true,
        }
    }

    pub fn try_submit(
        &mut self,
        year: PuzzleYear,
        day: PuzzleDay,
        part: Part,
        answer: &str,
        submit_fn: impl Fn(PuzzleYear, PuzzleDay, Part, &str) -> Result<String>,
    ) -> Result<bool> {
        if self.is_solved() {
            return Ok(true);
        }

        if !self.answer_is_ok(answer) {
            eprintln!("Answer was not valid. {answer} {self:?}");
            return Ok(false);
        }

        let response = submit_fn(year, day, part, answer)?;
        let already_done_re = Regex::new(r"already complete").unwrap();
        if already_done_re.is_match(&response) {
            *self = AnswerState::PreviouslyDone;
            return Ok(true);
        }
        eprintln!("{}", from_read(response.as_bytes(), 120));
        let wrong_re = Regex::new(r"not the right answer")?;
        if wrong_re.is_match(&response) {
            if let AnswerState::Attempts(attempts) = self {
                attempts.push(BadAnswers::from_response(&response, answer));
            }
            Ok(false)
        } else {
            *self = AnswerState::Solution(answer.to_owned());
            Ok(true)
        }
    }
}

impl Default for AnswerState {
    fn default() -> Self {
        AnswerState::Attempts(Vec::new())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PuzzleState {
    part_a: AnswerState,
    part_b: AnswerState,
}

impl PuzzleState {
    pub fn try_submit(
        &mut self,
        cookie: &str,
        year: PuzzleYear,
        day: PuzzleDay,
        answer: &PuzzleAnswer,
    ) -> Result<(bool, bool)> {
        self.try_submit_generic(year, day, answer, |year, day, part, answer| {
            submit_answer(cookie, year, day, part, answer)
        })
    }

    pub fn try_submit_generic(
        &mut self,
        year: PuzzleYear,
        day: PuzzleDay,
        answer: &PuzzleAnswer,
        submit_fn: impl Fn(PuzzleYear, PuzzleDay, Part, &str) -> Result<String>,
    ) -> Result<(bool, bool)> {
        let PuzzleAnswer(a, b) = answer;
        let a_correct = if let Some(answer_a) = a {
            self.part_a
                .try_submit(year, day, Part::A, answer_a, &submit_fn)?
        } else {
            false
        };

        let b_correct = if let Some(answer_b) = b {
            self.part_b
                .try_submit(year, day, Part::B, answer_b, &submit_fn)?
        } else {
            false
        };

        Ok((a_correct, b_correct))
    }

    pub fn load(year: PuzzleYear, day: PuzzleDay) -> Self {
        let dir = get_puzzle_dir(year, day);
        let path = dir.join("state.json");
        if let Ok(contents) = fs::read_to_string(path) {
            if let Ok(val) = serde_json::from_str(&contents) {
                return val;
            };
        };
        Default::default()
    }

    pub fn save(&self, year: PuzzleYear, day: PuzzleDay) -> Result<()> {
        let path = get_puzzle_dir(year, day).join("state.json");
        let contents = serde_json::to_string_pretty(self)?;
        fs::write(path, contents)?;
        Ok(())
    }
}
