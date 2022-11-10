use crate::utils::aocdata::{Part, TestCase};
use crate::utils::puzzle::{PuzzleFns, SolvePuzzle};
use crate::utils::read_lines;
use anyhow::Result;
use itertools::Itertools;
use ringbuffer::{AllocRingBuffer, RingBuffer, RingBufferExt, RingBufferRead, RingBufferWrite};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Default, Debug, Clone, Copy)]
struct AvgSum {
    nums: [i32; 3],
    start: usize,
    len: usize,
}

impl AvgSum {
    pub fn add(&mut self, x: i32) {
        let i = (self.start + self.len) % 3;
        if self.len == 3 {
            self.start = (self.start + 1) % 3
        } else {
            self.len += 1
        }
        self.nums[i] = x;
    }

    pub fn remove_first(&mut self) {
        if self.len == 0 {
            return;
        }
        if self.len == 1 {
            self.len = 0;
            self.start = 0;
        } else {
            self.len -= 1;
            self.start = (self.start + 1) % 3;
        }
    }

    pub fn sum(&self) -> i32 {
        let mut sum = 0;
        for i in 0..self.len {
            let real_index = (self.start + i) % 3;
            sum += self.nums[real_index];
        }
        return sum;
    }
}

pub struct Year2021Day1;

impl SolvePuzzle for Year2021Day1 {
    type Output = (i32, Option<i32>);

    fn puzzle_year_day() -> (i32, u32) {
        (2021, 1)
    }

    fn solve(input: &str) -> Result<Self::Output> {
        Ok((count_lines(input)?, None))
    }

    fn test_cases() -> Vec<TestCase> {
        vec![TestCase::new(
            Part::A,
            "199
200
208
210
200
207
240
269
260
263",
            7,
        )]
    }
}

#[test]
pub fn submit_year_2021_day_1() -> Result<()> {
    let tests = Year2021Day1::run_tests()?;
    let res = Year2021Day1::try_submit()?;
    eprintln!("{res:?}");
    Ok(())
}

pub fn count_lines(input: &str) -> Result<i32> {
    Ok(input
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count() as i32)
}

pub fn count_windows(input: &str) -> Result<i32> {
    Ok(input
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count() as i32)
}

#[test]
fn count_depths() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./src/year2021/day1input.txt") {
        let mut count = 0;

        let mut prev_sum: AvgSum = AvgSum::default();
        let mut curr_sum: AvgSum = AvgSum::default();
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(num) = line {
                let next: i32 = num.parse().unwrap();
                curr_sum.add(next);
                dbg!(next, curr_sum.sum());
                if prev_sum.len == 3 {
                    if curr_sum.sum() > prev_sum.sum() {
                        count += 1;
                    }
                }
                prev_sum = curr_sum;
            }
        }
        dbg!(count);
    }
}
