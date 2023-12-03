use crate::utils::{
    aocdata::{Part, TestCase},
    puzzle::{PuzzleFns, SolvePuzzle},
};
use anyhow::Result;
use itertools::Itertools;

pub struct AoC2023Day1;

fn naive_replace(line: &str) -> String {
    let mut line = line.to_string();
    line = line.replace("eightwone", "821");
    line = line.replace("oneightwo", "182");
    line = line.replace("twoneight", "218");
    line = line.replace("nineight", "98");
    line = line.replace("fiveight", "58");
    line = line.replace("oneight", "18");
    line = line.replace("threeight", "38");
    line = line.replace("twone", "21");
    line = line.replace("eightwo", "82");
    line = line.replace("one", "1");
    line = line.replace("two", "2");
    line = line.replace("three", "3");
    line = line.replace("four", "4");
    line = line.replace("five", "5");
    line = line.replace("six", "6");
    line = line.replace("seven", "7");
    line = line.replace("eight", "8");
    line.replace("nine", "9")
}

// fn smart_replace(line: &str) -> String {
//     let replacers = [("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"), ("five", "5"), ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9")];
//     let mut line = line.to_string();
//     loop {
//         let opts = replacers.into_iter().map(|(s, x)| {
//             (line.find(s), s, x)
//         }).filter(|a| a.0.is_some()).min_by(compare)
//     }

// }

impl SolvePuzzle for AoC2023Day1 {
    type Output = (Option<i32>, Option<i32>);
    fn puzzle_year_day() -> (i32, u32) {
        (2023, 1)
    }

    fn solve(input: &str) -> Result<Self::Output> {
        let x = input
            .lines()
            .map(|line| {
                let l = line.chars().find(|a| a.is_numeric()).unwrap_or('0');
                let r = line.chars().rev().find(|a| a.is_numeric()).unwrap_or('0');
                (l.to_string().parse::<i32>().unwrap() * 10) + r.to_string().parse::<i32>().unwrap()
            })
            .sum();

        let nums = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        let y = input
            .lines()
            .map(|line| {
                // let l_num = nums.map(|n| line.find(n)).iter().enumerate().reduce(|acc, elem| {
                //     if elem.1.is_some() {
                //         if acc.1.is_some() {
                //             if acc.1.unwrap() < elem.1.unwrap() {
                //                 acc
                //             } else {
                //                 elem
                //             }
                //         } else {
                //             elem
                //         }
                //     } else {
                //         acc
                //     }
                // });

                // let l_char = line.chars().enumerate().find(|(i, a)| a.is_numeric()).unwrap();
                // let l = {
                //     if let Some((num, pos)) = l_num {
                //         if
                //     }
                // }
                let mut line = naive_replace(line);

                let l = line.chars().find(|a| a.is_numeric()).unwrap();
                let r = line.chars().rev().find(|a| a.is_numeric()).unwrap();
                dbg!(
                    (l.to_string().parse::<i32>().unwrap() * 10)
                        + r.to_string().parse::<i32>().unwrap()
                )
            })
            .sum();

        Ok((Some(x), Some(y)))
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase::new(
                Part::A,
                "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet",
                142,
            ),
            TestCase::new(
                Part::B,
                "three1abc2
            pqr3stu8vwx
            a1b2c3d4e5fsix
            treb7uchet",
                163,
            ),
            TestCase::new(
                Part::B,
                "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen",
                281,
            ),
            // TestCase::new(Part::B, 0, 0),
        ]
    }
}
//29, 83, 13, 24, 42, 14, and 76
#[test]
fn day1() -> Result<()> {
    AoC2023Day1::run_tests()?;
    let res = AoC2023Day1::try_submit()?;
    eprintln!("{res:?}");
    Ok(())
}
