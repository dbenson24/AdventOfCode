use std::fmt::Display;

use crate::utils::{
    aocdata::{Part, TestCase},
    get_cardinal_neighbors,
    puzzle::{PuzzleFns, SolvePuzzle},
    Dir2, World,
};
use anyhow::{anyhow, Result};
use glam::IVec2;
use hashbrown::HashSet;
use itertools::Itertools;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive as Fp, ToPrimitive as Tp};

use super::intcode::IntcodeMachine;

pub struct AoC2019Day17;

#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
#[repr(u8)]
enum CamOut {
    Scaffold = 35,
    Space = 46,
    NewLine = 10,
    Up = b'^',
    Down = b'v',
    Left = b'<',
    Right = b'>',
    Tumbling = b'X',
}

impl Display for CamOut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = self.to_u8().unwrap() as char;
        x.fmt(f)
    }
}

impl CamOut {
    pub fn is_robot(&self) -> Option<Dir2> {
        match self {
            CamOut::Up => Some(Dir2::Down),
            CamOut::Down => Some(Dir2::Up),
            CamOut::Left => Some(Dir2::Left),
            CamOut::Right => Some(Dir2::Right),
            _ => None,
        }
    }

    pub fn is_scaffolding(&self) -> bool {
        match self {
            CamOut::Scaffold => true,
            CamOut::Up => true,
            CamOut::Down => true,
            CamOut::Left => true,
            CamOut::Right => true,
            _ => false,
        }
    }
}

pub fn get_cmd(a: Dir2, b: Dir2) -> &'static str {
    let a: IVec2 = a.into();
    let b: IVec2 = b.into();
    let angle = a.as_vec2().angle_between(b.as_vec2()).to_degrees();
    if (angle - 90.).abs() < 0.1 {
        return "R";
    }
    if (angle + 90.).abs() < 0.1 {
        return "L";
    }
    panic!("{:?}, {:?}", a, b);
}

impl SolvePuzzle for AoC2019Day17 {
    type Output = (Option<i32>, Option<i64>);
    fn puzzle_year_day() -> (i32, u32) {
        (2019, 17)
    }

    fn solve(input: &str) -> Result<Self::Output> {
        let mut numbers = IntcodeMachine::parse_str(input).unwrap();
        let (_input, output) = IntcodeMachine::spawn(numbers.clone());
        numbers[0] = 2;
        // input.send(Some(99));
        let view = output
            .iter()
            .filter(Option::is_some)
            .map(Option::unwrap)
            .map(|x| x as u8)
            .collect_vec();
        let string = String::from_utf8(view).unwrap();
        let mut robot_start_dir = Dir2::Up;
        let mut robot_start_pos = IVec2::ZERO;
        let mut world =
            World::<CamOut>::from_str(
                &string,
                |x| CamOut::from_u8(x.chars().nth(0).unwrap() as u8),
            );

        let sum: i32 = world
            .world
            .iter()
            .map(|(pos, cam)| {
                if let Some(dir) = cam.is_robot() {
                    robot_start_pos = *pos;
                    robot_start_dir = dir;
                }
                if cam.is_scaffolding() {
                    let neighbors = get_cardinal_neighbors(*pos);
                    let intersection = neighbors
                        .iter()
                        .map(|pos| {
                            if let Some(x) = world.world.get(pos) {
                                if x.is_scaffolding() {
                                    true
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        })
                        .filter(|x| *x)
                        .count()
                        > 2;
                    if intersection {
                        dbg!(pos);
                        pos.x * pos.y
                    } else {
                        0
                    }
                } else {
                    0
                }
            })
            .sum();

        world.pretty_print(false);

        let next_dir = |pos: IVec2, visited: &HashSet<IVec2>| {
            let neighbors = get_cardinal_neighbors(pos);
            let next = neighbors
                .iter()
                .map(|neigh| {
                    if let Some(x) = world.world.get(neigh) {
                        if x.is_scaffolding() && !visited.contains(neigh) {
                            Some(*neigh)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .filter(Option::is_some)
                .map(Option::unwrap)
                .nth(0);
            if let Some(next) = next {
                let next_dir: Dir2 = (next - pos).into();
                Some(next_dir)
            } else {
                None
            }
        };

        let next_pos = |pos: IVec2, dir: Dir2| {
            let mut next = pos;
            let offset: IVec2 = dir.into();
            loop {
                if let Some(x) = world.world.get(&(next + offset)) {
                    if x.is_scaffolding() {
                        next = next + offset;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            next
        };
        let mut visited = HashSet::new();
        let mut dir = robot_start_dir;
        let mut pos = robot_start_pos;
        while let Some(next_dir) = next_dir(pos, &visited) {
            let cmd = get_cmd(dir, next_dir);
            let next_pos = next_pos(pos, next_dir);
            let dist = (next_pos - pos).abs().max_element();
            print!("{cmd}{dist}");
            let next_offset: IVec2 = next_dir.into();
            visited.insert(next_pos - next_offset);
            visited.insert(next_pos);
            pos = next_pos;
            dir = next_dir;
        }
        println!("");
        let (input, output) = IntcodeMachine::spawn(numbers.clone());
        IntcodeMachine::send_str(&input, "A,A,B,C,B,C,B,C,B,A\n");
        IntcodeMachine::send_str(&input, "L,10,L,8,R,8,L,8,R,6\n");
        IntcodeMachine::send_str(&input, "R,6,R,8,R,8\n");
        IntcodeMachine::send_str(&input, "R,6,R,6,L,8,L,10\n");
        IntcodeMachine::send_str(&input, "n\n");

        let x = output.iter().last().unwrap();

        Ok((Some(sum), x))
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
    AoC2019Day17::run_tests()?;
    let res = AoC2019Day17::try_submit()?;
    eprintln!("{res:?}");
    Ok(())
}

/*
L10L8R8L8R6L10L8R8L8R6R6R8R8R6R6L8L10R6R8R8R6R6L8L10R6R8R8R6R6L8L10R6R8R8L10L8R8L8R6
A: L10L8R8L8R6
B: R6R8R8
C: R6R6L8L10
AABCBCBCBA

*/
