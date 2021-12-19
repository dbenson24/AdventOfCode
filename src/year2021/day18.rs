use crate::utils::*;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric0, alphanumeric1, char, one_of},
    combinator::recognize,
    multi::{many0, many1},
    sequence::pair,
    IResult,
};
use rayon::iter::*;
use std::{cell::RefCell, rc::Rc, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_json::Result;
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum FishVal {
    Val(i64),
    Child(Box<SnailFish>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SnailFish(pub FishVal, pub FishVal);

impl FishVal {
    pub fn val(&self) -> i64 {
        match self {
            FishVal::Child(child) => child.magnitude(),
            &FishVal::Val(a) => a,
        }
    }
}

impl SnailFish {
    pub fn magnitude(&self) -> i64 {
        (3 * self.0.val()) + (2 * self.1.val())
    }

    pub fn reduce(&mut self) {
        loop {
            if let Some(_) = self.explode(0) {
                continue;
            }

            if self.split() {
                continue;
            }

            break;
        }
    }

    pub fn explode(&mut self, depth: usize) -> Option<(bool, i64, i64)> {
        if let FishVal::Val(a) = self.0 {
            if let FishVal::Val(b) = self.1 {
                if depth >= 4 {
                    return Some((true, a, b));
                }
                return None;
            }
        }

        if let FishVal::Child(left) = &mut self.0 {
            if let Some((delete, add_l, mut add_r)) = left.explode(depth + 1) {
                if delete {
                    self.0 = FishVal::Val(0)
                }
                if add_r > 0 {
                    match &mut self.1 {
                        FishVal::Val(b) => {
                            self.1 = FishVal::Val(*b + add_r);
                            add_r = 0
                        }
                        FishVal::Child(right) => {
                            if right.add_to_left_most(add_r) {
                                add_r = 0
                            }
                        }
                    }
                }
                return Some((false, add_l, add_r));
            }
        }
        if let FishVal::Child(right) = &mut self.1 {
            if let Some((delete, mut add_l, add_r)) = right.explode(depth + 1) {
                if delete {
                    self.1 = FishVal::Val(0)
                }
                if add_l > 0 {
                    match &mut self.0 {
                        FishVal::Val(b) => {
                            self.0 = FishVal::Val(*b + add_l);
                            add_l = 0
                        }
                        FishVal::Child(left) => {
                            if left.add_to_right_most(add_l) {
                                add_l = 0
                            }
                        }
                    }
                }
                return Some((false, add_l, add_r));
            }
        }
        None
    }

    pub fn split(&mut self) -> bool {
        match &mut self.0 {
            FishVal::Val(a) => {
                if *a > 9 {
                    let l = FishVal::Val(*a / 2);
                    let r = FishVal::Val(*a - (*a / 2));
                    let child = SnailFish(l, r);
                    self.0 = FishVal::Child(Box::new(child));
                    return true;
                }
            }
            FishVal::Child(child) => {
                if child.split() {
                    return true;
                }
            }
        };

        match &mut self.1 {
            FishVal::Val(a) => {
                if *a > 9 {
                    let l = FishVal::Val(*a / 2);
                    let r = FishVal::Val(*a - (*a / 2));
                    let child = SnailFish(l, r);
                    self.1 = FishVal::Child(Box::new(child));
                    return true;
                }
            }
            FishVal::Child(child) => {
                if child.split() {
                    return true;
                }
            }
        };
        false
    }

    pub fn add_to_left_most(&mut self, val: i64) -> bool {
        match &mut self.0 {
            FishVal::Val(a) => {
                self.0 = FishVal::Val(*a + val);
                return true;
            }
            FishVal::Child(left) => {
                if left.add_to_left_most(val) {
                    return true;
                }
            }
        }
        return false;
    }
    pub fn add_to_right_most(&mut self, val: i64) -> bool {
        match &mut self.1 {
            FishVal::Val(a) => {
                self.1 = FishVal::Val(*a + val);
                return true;
            }
            FishVal::Child(right) => {
                if right.add_to_right_most(val) {
                    return true;
                }
            }
        }
        return false;
    }
}

pub fn parse_snail(text: &str) -> Result<SnailFish> {
    serde_json::from_str(text)
}

#[test]
pub fn test_1() {
    dbg!(parse_snail("[1,2]"));
}

#[test]
pub fn test_2() {
    dbg!(parse_snail("[[1,2],3]"));
}

#[test]
pub fn test_3() {
    dbg!(parse_snail("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"));
}

#[test]
pub fn test_4() {
    let input = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]";
    let mut num = parse_snail(input).unwrap();
    let mut num2 = num.clone();
    dbg!(serde_json::to_string(&num));
    num.explode(0);
    dbg!(serde_json::to_string(&num));
    num.explode(0);
    dbg!(serde_json::to_string(&num));
    num.split();
    dbg!(serde_json::to_string(&num));
    num.split();
    dbg!(serde_json::to_string(&num));
    num.explode(0);
    dbg!(serde_json::to_string(&num));

    num2.reduce();
    assert_eq!(
        serde_json::to_string(&num2).unwrap(),
        serde_json::to_string(&num).unwrap()
    )
}

#[test]
pub fn test5() {
    let snail = parse_snail("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").unwrap();
    dbg!(snail.magnitude());
}

#[test]
pub fn base() {
    if let Ok(lines) = read_lines("./src/year2021/data/day18input.txt") {
        // Consumes the iterator, returns an (Optional) String

        let mut snails: Vec<_> = lines
            .map(|l| l.unwrap())
            .map(|l| parse_snail(&l).unwrap())
            .collect();
        let mut snails_iter = snails.clone().into_iter();

        let first = snails_iter.next().unwrap();
        let sum = snails_iter.fold(first, |acc, snail| {
            let mut acc = SnailFish(
                FishVal::Child(Box::new(acc)),
                FishVal::Child(Box::new(snail)),
            );
            acc.reduce();
            acc
        });
        dbg!(sum.magnitude());

        let max = snails
            .iter()
            .permutations(2)
            .map(|x| {
                let mut iter = x.into_iter();
                let first = iter.next().unwrap();
                let sum = iter.fold(first.clone(), |acc, snail| {
                    let mut acc = SnailFish(
                        FishVal::Child(Box::new(acc)),
                        FishVal::Child(Box::new(snail.clone())),
                    );
                    acc.reduce();
                    acc
                });
                sum.magnitude()
            })
            .max()
            .unwrap();

        dbg!(max);
    }
}
