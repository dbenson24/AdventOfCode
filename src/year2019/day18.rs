use std::{
    convert::{TryFrom, TryInto},
    fmt::Display,
};

use crate::utils::{
    aocdata::{Part, TestCase},
    djikstra, get_cardinal_neighbors,
    puzzle::{PuzzleFns, SolvePuzzle},
    World,
};
use anyhow::{anyhow, Error, Result};
use glam::IVec2;
use hashbrown::HashSet;
use itertools::Itertools;
use smallvec::SmallVec;
use tinyvec::TinyVec;

pub struct AoC2019Day18;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Player,
    Key(char),
    Door(char),
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Empty),
            '@' => Ok(Tile::Player),
            '#' => Ok(Tile::Wall),
            _ => {
                if value >= 'a' && value <= 'z' {
                    Ok(Tile::Key(value))
                } else if value >= 'A' && value <= 'Z' {
                    Ok(Tile::Door(value))
                } else {
                    Err(anyhow!("Invalid tile {value}"))
                }
            }
        }
    }
}

impl Tile {
    pub fn get_char(&self) -> char {
        match self {
            Tile::Empty => '.',
            Tile::Wall => '#',
            Tile::Player => '@',
            Tile::Key(x) => *x,
            Tile::Door(x) => *x,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get_char().fmt(f)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct SearchState {
    pub pos: IVec2,
    pub keys: TinyVec<[char; 8]>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct SearchStateB {
    pub pos: [IVec2; 4],
    pub keys: TinyVec<[char; 8]>,
}

impl SearchState {
    pub fn add_key(&mut self, key: char) {
        match self.keys.binary_search(&key) {
            Ok(_) => (),
            Err(idx) => self.keys.insert(idx, key),
        }
    }

    pub fn has_key(&self, key: char) -> bool {
        self.keys.binary_search(&key).is_ok()
    }

    pub fn with_pos(&self, pos: IVec2) -> Self {
        let mut next = self.clone();
        next.pos = pos;
        next
    }
}

impl SearchStateB {
    pub fn add_key(&mut self, key: char) {
        match self.keys.binary_search(&key) {
            Ok(_) => (),
            Err(idx) => self.keys.insert(idx, key),
        }
    }

    pub fn has_key(&self, key: char) -> bool {
        self.keys.binary_search(&key).is_ok()
    }

    pub fn with_pos(&self, idx: usize, pos: IVec2) -> Self {
        let mut next = self.clone();
        next.pos[idx] = pos;
        next
    }
}

impl SolvePuzzle for AoC2019Day18 {
    type Output = (Option<usize>, Option<usize>);
    fn puzzle_year_day() -> (i32, u32) {
        (2019, 18)
    }
    fn solve(input: &str) -> Result<Self::Output> {
        let mut world: World<Tile> = input.try_into()?;

        world.pretty_print(false);
        let pos = *world
            .world
            .iter()
            .find(|(_, t)| **t == Tile::Player)
            .unwrap()
            .0;
        let start = SearchState {
            pos,
            keys: TinyVec::new(),
        };
        let key_count: usize = world
            .world
            .iter()
            .map(|(_, t)| match t {
                Tile::Key(_) => 1,
                _ => 0,
            })
            .sum();
        dbg!(key_count);
        let cost = djikstra(
            start,
            0,
            &mut (),
            |x, _curr_cost, _| x.keys.len() == key_count,
            |x, _cost, _| {
                let mut next_states: SmallVec<[Option<(SearchState, usize)>; 8]> = SmallVec::new();
                world.min_cost_4(
                    &mut next_states,
                    x.pos,
                    |tile_pos, world, next_states, cost| {
                        if let Some(tile) = world.world.get(&tile_pos) {
                            match tile {
                                Tile::Key(key) => {
                                    if !x.has_key(*key) {
                                        let mut next = x.with_pos(tile_pos);
                                        next.add_key(*key);
                                        next_states.push(Some((next, cost)));
                                    }
                                }
                                _ => (),
                            }
                        };
                        false
                    },
                    |pos, world, _| {
                        if let Some(tile) = world.world.get(&pos) {
                            match tile {
                                Tile::Door(door) => {
                                    if x.has_key(door.to_ascii_lowercase()) {
                                        Some(1)
                                    } else {
                                        None
                                    }
                                }
                                Tile::Wall => None,
                                _ => Some(1),
                            }
                        } else {
                            None
                        }
                    },
                );
                next_states
            },
        );

        let mut b_world = world.clone();
        let mid_x = pos.x;
        let mid_y = pos.y;
        b_world.world.insert(IVec2::new(mid_x, mid_y), Tile::Wall);
        b_world
            .world
            .insert(IVec2::new(mid_x - 1, mid_y), Tile::Wall);
        b_world
            .world
            .insert(IVec2::new(mid_x + 1, mid_y), Tile::Wall);
        b_world
            .world
            .insert(IVec2::new(mid_x, mid_y - 1), Tile::Wall);
        b_world
            .world
            .insert(IVec2::new(mid_x, mid_y + 1), Tile::Wall);
        b_world.pretty_print(false);

        let start2 = SearchStateB {
            pos: [
                IVec2::new(mid_x - 1, mid_y - 1),
                IVec2::new(mid_x + 1, mid_y - 1),
                IVec2::new(mid_x - 1, mid_y + 1),
                IVec2::new(mid_x + 1, mid_y + 1),
            ],
            keys: TinyVec::new(),
        };

        let costb = djikstra(
            start2,
            0,
            &mut (),
            |x, _, _| x.keys.len() == key_count,
            |x, _, _| {
                let mut next_states: SmallVec<[Option<(SearchStateB, usize)>; 8]> = SmallVec::new();
                x.pos.iter().enumerate().for_each(|(idx, robot_pos)| {
                    b_world.min_cost_4(
                        &mut next_states,
                        *robot_pos,
                        |tile_pos, world, next_states, cost| {
                            if let Some(tile) = world.world.get(&tile_pos) {
                                match tile {
                                    Tile::Key(key) => {
                                        if !x.has_key(*key) {
                                            let mut next = x.with_pos(idx, tile_pos);
                                            next.add_key(*key);
                                            next_states.push(Some((next, cost)));
                                        }
                                    }
                                    _ => (),
                                }
                            };
                            false
                        },
                        |pos, world, _| {
                            if let Some(tile) = world.world.get(&pos) {
                                match tile {
                                    Tile::Door(door) => {
                                        if x.has_key(door.to_ascii_lowercase()) {
                                            Some(1)
                                        } else {
                                            None
                                        }
                                    }
                                    Tile::Wall => None,
                                    _ => Some(1),
                                }
                            } else {
                                None
                            }
                        },
                    );
                });

                next_states
            },
        );

        Ok((cost, costb))
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase::new(
                Part::A,
                "#########
#b.A.@.a#
#########",
                8,
            ),
            TestCase::new(
                Part::A,
                "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################",
                81,
            ),
            TestCase::new(
                Part::B,
                "#######
#a.#Cd#
##...##
##.@.##
##...##
#cB#Ab#
#######",
                8,
            ),
            TestCase::new(
                Part::B,
                "###############
#d.ABC.#.....a#
######...######
######.@.######
######...######
#b.....#.....c#
###############",
                24,
            ),
            TestCase::new(
                Part::B,
                "#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba...BcIJ#
#####.@.#####
#nK.L...G...#
#M###N#H###.#
#o#m..#i#jk.#
#############",
                72,
            ),
        ]
    }
}

#[test]
fn test() -> Result<()> {
    AoC2019Day18::run_tests()?;
    let res = AoC2019Day18::try_submit()?;
    eprintln!("{res:?}");
    Ok(())
}
