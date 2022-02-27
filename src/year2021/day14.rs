use rayon::iter::*;
use rayon::slice::ParallelSlice;
use rustc_hash::FxHashMap;
use std::collections::{HashMap, HashSet};
use std::str;

use crate::utils::*;

pub fn day_14_part_1() {
    if let Ok(lines) = read_lines("./src/year2021/data/day14input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut first_line = true;
        let mut mapping: FxHashMap<(u8, u8), u8> = FxHashMap::default();
        let mut bytes = vec![];
        for (_line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                if first_line {
                    first_line = false;
                    bytes = contents.as_bytes().iter().map(|x| *x).collect();
                } else {
                    let pair: Vec<_> = contents.split(" -> ").collect();
                    if pair.len() > 1 {
                        let bytes = pair[0].as_bytes();
                        mapping.insert((bytes[0], bytes[1]), pair[1].as_bytes()[0]);
                    }
                }
            }
        }

        let inserted = (0..40).fold(bytes, |bs, i| {
            let mut x: Vec<_> = bs
                .par_windows(2)
                .fold(
                    || Vec::new(),
                    |mut acc: Vec<u8>, x| {
                        acc.push(x[0]);
                        if let Some(byte) = mapping.get(&(x[0], x[1])) {
                            acc.push(*byte);
                        }
                        acc
                    },
                )
                .flat_map(|x| x)
                .collect();
            x.push(bs[bs.len() - 1]);
            dbg!(i, x.len());

            x
        });

        let counts = inserted.iter().fold(FxHashMap::default(), |mut acc, x| {
            if let Some(val) = acc.get(x) {
                acc.insert(x, val + 1);
            } else {
                acc.insert(x, 1);
            };
            acc
        });

        dbg!(counts.values().max().unwrap() - counts.values().min().unwrap());
    }
}

pub fn solve_window(
    window: (u8, u8),
    rounds: usize,
    mapping: &FxHashMap<(u8, u8), u8>,
    memoized: &mut FxHashMap<((u8, u8), usize), FxHashMap<u8, i64>>,
) -> FxHashMap<u8, i64> {
    if rounds == 0 {
        let mut counts = FxHashMap::default();
        counts.insert(window.0, 1i64);
        return counts;
    }

    if let Some(res) = memoized.get(&(window, rounds)) {
        res.clone()
    } else {
        if let Some(byte) = mapping.get(&window) {
            let counts_left = solve_window((window.0, *byte), rounds - 1, mapping, memoized);
            let counts_right = solve_window((*byte, window.1), rounds - 1, mapping, memoized);
            let res = counts_right
                .into_iter()
                .fold(counts_left, |mut acc, (key, count)| {
                    if let Some(val) = acc.get(&key) {
                        acc.insert(key, count + *val);
                    } else {
                        acc.insert(key, count);
                    }
                    acc
                });

            memoized.insert((window, rounds), res.clone());
            res
        } else {
            let mut counts = FxHashMap::default();
            counts.insert(window.0, 1i64);
            counts.insert(window.1, 1);
            memoized.insert((window, rounds), counts.clone());

            counts
        }
    }
}

pub fn day_14_part_2() {
    if let Ok(lines) = read_lines("./src/year2021/data/day14input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut first_line = true;
        let mut mapping: FxHashMap<(u8, u8), u8> = FxHashMap::default();
        let mut bytes = vec![];
        for (_line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                if first_line {
                    first_line = false;
                    bytes = contents.as_bytes().iter().map(|x| *x).collect();
                } else {
                    let pair: Vec<_> = contents.split(" -> ").collect();
                    if pair.len() > 1 {
                        let bytes = pair[0].as_bytes();
                        mapping.insert((bytes[0], bytes[1]), pair[1].as_bytes()[0]);
                    }
                }
            }
        }

        let mut memoized = FxHashMap::default();

        let counts = bytes.windows(2).fold(FxHashMap::default(), |acc, window| {
            let counts = solve_window((window[0], window[1]), 40, &mapping, &mut memoized);
            counts.into_iter().fold(acc, |mut acc, (key, count)| {
                if let Some(val) = acc.get(&key) {
                    acc.insert(key, count + *val);
                } else {
                    acc.insert(key, count);
                }
                acc
            })
        });
        dbg!(memoized.len());

        dbg!(counts.values().max().unwrap() - counts.values().min().unwrap());
    }
}
