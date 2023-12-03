use std::fmt::Display;

use crate::utils::{
    aocdata::{Part, TestCase},
    get_neighbors,
    puzzle::{PuzzleFns, SolvePuzzle},
    World,
};
use anyhow::Result;
use glam::IVec2;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

pub struct AoC2023Day3;

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Blank,
    Digit(u32),
    Symbol,
    Gear,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Blank => f.write_str("."),
            Tile::Digit(x) => f.write_fmt(format_args!("{x}")),
            Tile::Symbol => f.write_str("x"),
            Tile::Gear => f.write_str("*"),
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value == '.' {
            Ok(Self::Blank)
        } else if value.is_numeric() {
            Ok(Self::Digit(value.to_string().parse().unwrap()))
        } else if value == '*' {
            Ok(Self::Gear)
        } else {
            Ok(Self::Symbol)
        }
    }
}

fn part_numbers(world: &World<Tile>) -> (u32, u32) {
    let mut ratios = HashMap::new();
    let mut ratio_counts: HashMap<IVec2, u32> = HashMap::new();
    for (key, val) in world.world.iter() {
        if val == &Tile::Gear {
            ratios.insert(*key, 1);
        }
    }
    let mut sum = 0;
    for y in world.min_y()..=world.max_y() {
        let mut num: Option<u32> = None;
        let mut keep = false;
        let mut affected_gears = HashSet::new();
        for x in world.min_x()..=world.max_x() {
            let pos = IVec2::new(x, y);
            let cell = &world.world[&pos];
            match cell {
                Tile::Blank | Tile::Symbol | Tile::Gear => {
                    if let Some(curr_num) = num {
                        if keep {
                            sum += curr_num;
                        }
                        for gear_pos in &affected_gears {
                            let curr_ratio = ratios[gear_pos];
                            ratios.insert(*gear_pos, curr_ratio * curr_num);
                            let curr_count = ratio_counts.get(gear_pos).unwrap_or(&0);
                            ratio_counts.insert(*gear_pos, *curr_count + 1);
                        }
                    }
                    keep = false;
                    num = None;
                    affected_gears.clear();
                }
                Tile::Digit(x) => {
                    if let Some(curr_num) = num {
                        num = Some((curr_num * 10) + *x)
                    } else {
                        num = Some(*x)
                    }
                    for neigh in get_neighbors(pos) {
                        if let Some(n) = world.world.get(&neigh) {
                            if n == &Tile::Symbol {
                                keep = true;
                            }
                            if n == &Tile::Gear {
                                keep = true;
                                affected_gears.insert(neigh);
                            }
                        }
                    }
                }
            }
        }
        if keep {
            if let Some(curr_num) = num {
                sum += curr_num;

                for gear_pos in &affected_gears {
                    let curr_ratio = ratios[gear_pos];
                    ratios.insert(*gear_pos, curr_ratio * curr_num);
                    let curr_count = ratio_counts.get(gear_pos).unwrap_or(&0);
                    ratio_counts.insert(*gear_pos, *curr_count + 1);
                }
            }
        }
    }
    dbg!(&ratio_counts);
    let ratio_sum = ratio_counts
        .iter()
        .filter(|(_, &count)| count == 2)
        .map(|(gear_pos, _count)| dbg!(ratios[gear_pos]))
        .sum();
    (sum, ratio_sum)
}

impl SolvePuzzle for AoC2023Day3 {
    type Output = (u32, u32);
    fn puzzle_year_day() -> (i32, u32) {
        (2023, 3)
    }

    fn solve(input: &str) -> Result<Self::Output> {
        let x: World<Tile> = input.try_into()?;

        Ok(part_numbers(&x))
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase::new(
                Part::A,
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
                4361,
            ),
            TestCase::new(
                Part::B,
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
                467835,
            ),
            // TestCase::new(Part::B, 0, 0),
        ]
    }
}

#[test]
fn day1() -> Result<()> {
    AoC2023Day3::run_tests()?;
    let res = AoC2023Day3::try_submit()?;
    eprintln!("{res:?}");
    Ok(())
}
