use crate::utils::{
    aocdata::{Part, TestCase},
    puzzle::{PuzzleFns, SolvePuzzle},
};
use anyhow::Result;
use itertools::Itertools;
use rayon::iter::ParallelBridge;
use rayon::prelude::*;

pub struct AoC2023Day5;

#[derive(Debug, Clone, Copy)]
pub struct NumberTransform {
    pub source: i64,
    pub target: i64,
    pub len: i64,
}

impl NumberTransform {
    pub fn new(target: i64, source: i64, len: i64) -> Self {
        Self {
            source,
            target,
            len,
        }
    }
}

pub fn transform(x: i64, transforms: &[NumberTransform]) -> i64 {
    for transform in transforms {
        let pos = x - transform.source;
        if pos >= 0 && pos < transform.len {
            return transform.target + pos;
        }
    }
    x
}

impl SolvePuzzle for AoC2023Day5 {
    type Output = (Option<i64>, Option<i64>);
    fn puzzle_year_day() -> (i32, u32) {
        (2023, 5)
    }

    fn solve(input: &str) -> Result<Self::Output> {
        let mut lines = input.lines();
        let seed_line = lines.next().unwrap();
        let (_, seeds) = seed_line.split_once(": ").unwrap();
        let seeds = seeds
            .split(" ")
            .map(|s| s.parse::<i64>().unwrap())
            .collect_vec();
        dbg!(&seeds);
        lines.next();
        lines.next();
        let mut maps = Vec::new();
        let mut curr_map = Vec::new();
        while let Some(line) = lines.next() {
            if line.len() < 2 {
                if curr_map.len() > 0 {
                    maps.push(curr_map);
                    curr_map = Vec::new();
                }
                continue;
            }
            if line.chars().next().unwrap().is_alphabetic() {
                continue;
            }
            let mut nums = line.split(" ").map(|x| x.parse().unwrap());
            let transform = NumberTransform::new(
                nums.next().unwrap(),
                nums.next().unwrap(),
                nums.next().unwrap(),
            );
            curr_map.push(transform)
        }
        if curr_map.len() > 0 {
            maps.push(curr_map);
        }

        let mut part_a_seeds = seeds.clone();
        for map in &maps {
            for seed in part_a_seeds.iter_mut() {
                *seed = transform(*seed, map);
            }
        }
        let part_a = *part_a_seeds.iter().min().unwrap();

        let seeds_b = input.lines().next().unwrap();
        let (_, seeds) = seeds_b.split_once(": ").unwrap();

        let part_b = seeds
            .split(" ")
            .tuples()
            .flat_map(|(start, len)| {
                let start: i64 = start.parse().unwrap();
                let len: i64 = len.parse().unwrap();
                start..(start + len)
            })
            .enumerate()
            .par_bridge()
            .map(|(idx, mut seed)| {
                if idx % 2_i64.pow(24) as usize == 0 {
                    dbg!(idx);
                }
                for map in &maps {
                    seed = transform(seed, map);
                }
                seed
            })
            .min();

        Ok((Some(part_a), part_b))
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase::new(
                Part::A,
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
                35,
            ),
            TestCase::new(
                Part::B,
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
                46,
            ), // TestCase::new(Part::B, 0, 0),
        ]
    }
}

#[test]
fn day1() -> Result<()> {
    AoC2023Day5::run_tests()?;
    let res = AoC2023Day5::try_submit()?;
    eprintln!("{res:?}");
    Ok(())
}
