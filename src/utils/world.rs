use super::{djikstra, djikstra_path, get_cardinal_neighbors, read_lines};
use anyhow::{anyhow, Error};
pub use glam::{IVec2, IVec3, Quat, Vec2, Vec3};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::*;
use pad::PadStr;
pub use smallvec::{smallvec, SmallVec};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::convert::{TryFrom, TryInto};
use std::fmt::{Debug, Display};
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct World<T> {
    pub world: HashMap<IVec2, T>,
}

impl<T> TryFrom<&str> for World<T>
where
    T: TryFrom<char>,
    <T as TryFrom<char>>::Error: Debug,
{
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(World::<T>::from_str(value, |x| {
            let c: char = x.chars().nth(0).unwrap();
            let t: T = c.try_into().unwrap();
            t
        }))
    }
}

impl<T> World<T> {
    pub fn from_file<U: Into<Option<T>>>(path: &str, parse_fn: impl Fn(&str) -> U) -> Option<Self> {
        if let Ok(lines) = read_lines(path) {
            let mut world = HashMap::new();
            for (y, line) in lines.enumerate() {
                if let Ok(contents) = line {
                    for (x, height) in contents
                        .split("")
                        .filter(|s| s.len() > 0)
                        .map(&parse_fn)
                        .map(|x| {
                            let x: Option<T> = x.into();
                            x
                        })
                        .enumerate()
                        .filter(|(_i, x)| x.is_some())
                        .map(|(i, x)| (i, x.unwrap()))
                    {
                        let pos = IVec2::new(x as i32, y as i32);
                        world.insert(pos, height);
                    }
                }
            }
            Some(World { world })
        } else {
            None
        }
    }

    pub fn from_str<U: Into<Option<T>>>(input: &str, parse_fn: impl Fn(&str) -> U) -> Self {
        let mut world = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, height) in line
                .split("")
                .filter(|s| s.len() > 0)
                .map(&parse_fn)
                .map(|x| {
                    let x: Option<T> = x.into();
                    x
                })
                .enumerate()
                .filter(|(_i, x)| x.is_some())
                .map(|(i, x)| (i, x.unwrap()))
            {
                let pos = IVec2::new(x as i32, y as i32);
                world.insert(pos, height);
            }
        }
        World { world }
    }

    pub fn pretty_print(&self, rev_y: bool)
    where
        T: Display,
    {
        self.pretty_print_custom(|x, _| x.to_string(), rev_y)
    }

    pub fn pretty_print_custom(&self, str_fn: impl Fn(&T, IVec2) -> String, rev_y: bool) {
        let max_y = self.max_y();
        let max_x = self.max_x();
        let min_x = self.min_x().min(0);
        let min_y = self.min_y().min(0);

        if rev_y {
            for y in (min_y..=max_y).rev() {
                let mut row = vec![];
                for x in min_x..=max_x {
                    let pos = IVec2::new(x, y);
                    if let Some(val) = self.world.get(&pos) {
                        row.push(str_fn(val, pos));
                    } else {
                        row.push(" ".to_string());
                    }
                }
                println!("{}", row.iter().join(""));
            }
        } else {
            for y in min_y..=max_y {
                let mut row = vec![];
                for x in min_x..=max_x {
                    let pos = IVec2::new(x, y);
                    if let Some(val) = self.world.get(&pos) {
                        row.push(str_fn(val, pos));
                    } else {
                        row.push(" ".to_string());
                    }
                }
                println!("{}", row.iter().join(""));
            }
        }
    }

    pub fn max_x(&self) -> i32 {
        self.world.keys().map(|pos| pos.x).max().unwrap()
    }
    pub fn max_y(&self) -> i32 {
        self.world.keys().map(|pos| pos.y).max().unwrap()
    }

    pub fn min_x(&self) -> i32 {
        self.world.keys().map(|pos| pos.x).min().unwrap()
    }
    pub fn min_y(&self) -> i32 {
        self.world.keys().map(|pos| pos.y).min().unwrap()
    }

    /***
     * Calculates the minimum cost to move from the start position to the state determined by the done fn.
     * If done never returns true this function returns None. The weight function should return None for
     * any position that cannot be traversed.
     */
    pub fn min_cost_4<S>(
        &mut self,
        state: &mut S,
        start: IVec2,
        done: impl Fn(IVec2, &mut Self, &mut S, usize) -> bool,
        get_weight: impl Fn(IVec2, &mut Self, &mut S) -> Option<usize>,
    ) -> Option<usize> {
        djikstra(
            start,
            0,
            &mut (self, state),
            |&pos, cost, (world, extra_state)| done(pos, world, extra_state, cost),
            |pos, _, (world, extra_state)| {
                let x: SmallVec<[_; 4]> = get_cardinal_neighbors(*pos)
                    .iter()
                    .map(|x| {
                        if let Some(w) = get_weight(*x, world, extra_state) {
                            Some((*x, w))
                        } else {
                            None
                        }
                    })
                    .collect();
                x
            },
        )
    }

    /***
     * Calculates the lowest cost path to move from the start position to the state determined by the done fn.
     * If done never returns true this function returns None. The weight function should return None for
     * any position that cannot be traversed.
     */
    pub fn min_path_4<S>(
        &mut self,
        extra_state: &mut S,
        start: IVec2,
        done: impl Fn(IVec2, &mut Self, &mut S) -> bool,
        get_weight: impl Fn(IVec2, &mut Self, &mut S) -> Option<usize>,
    ) -> Option<(usize, Vec<IVec2>)> {
        djikstra_path(
            start,
            0,
            &mut (self, extra_state),
            |&pos, _, (state, extra_state)| done(pos, state, extra_state),
            |pos, _, (state, extra_state)| {
                let x: SmallVec<[_; 4]> = get_cardinal_neighbors(*pos)
                    .iter()
                    .map(|x| {
                        if let Some(w) = get_weight(*x, state, extra_state) {
                            Some((*x, w))
                        } else {
                            None
                        }
                    })
                    .collect();
                x
            },
        )
    }
}
