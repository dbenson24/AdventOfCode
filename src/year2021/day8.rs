use std::collections::BTreeMap;

use nom::Map;

use crate::utils::*;

use std::convert::From;
use std::str::FromStr;

use itertools::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum SIG {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Debug)]
struct ParseSigError;

impl FromStr for SIG {
    type Err = ParseSigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(SIG::A),
            "b" => Ok(SIG::B),
            "c" => Ok(SIG::C),
            "d" => Ok(SIG::D),
            "e" => Ok(SIG::E),
            "f" => Ok(SIG::F),
            "g" => Ok(SIG::G),
            _ => Err(ParseSigError),
        }
    }
}

static mappings: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

#[derive(Debug, Clone, Default)]
pub struct Mapping {
    pub dat: [[bool; 7]; 10],
}

pub fn str_to_sigs(str: &str, sigs: &mut [bool; 7]) {
    str.split("").filter(|s| s.len() > 0).for_each(|s| {
        let sig: SIG = s.parse().unwrap();
        sigs[sig as usize] = true
    })
}

impl Mapping {
    pub fn new(signals: &[&str]) -> Self {
        let mut dat: [[bool; 7]; 10] = Default::default();
        signals.iter().enumerate().for_each(|(i, sigs)| {
            str_to_sigs(sigs, &mut dat[i]);
        });
        Mapping { dat }
    }

    pub fn convert(&self, order: &[usize]) -> Mapping {
        let mut dat: [[bool; 7]; 10] = Default::default();

        for (i, sig) in self.dat.iter().enumerate() {
            for (j, &x) in sig.iter().enumerate() {
                dat[i][order[j]] = x
            }
        }

        Mapping { dat }
    }

    pub fn same(&self, other: &Mapping) -> bool {
        self.dat.iter().all(|sigs| other.dat.contains(sigs))
    }

    pub fn order_map(&self, other: &Mapping, mapping: &[usize]) -> Mapping {
        let adjusted = self.convert(mapping);
        let mut ordered_map: Mapping = Mapping::default();
        for (i, &sigs) in adjusted.dat.iter().enumerate() {
            ordered_map.dat[other.dat.iter().position(|&x| x == sigs).unwrap()] = self.dat[i]
        }

        ordered_map
    }
}

pub fn map_signals<'a>(signals: &Vec<&'a str>) -> Mapping {
    let correct = Mapping::new(&mappings);
    let new = Mapping::new(&signals);

    let mapping = (0..7 as usize)
        .permutations(7)
        .filter(|perm| {
            let adjusted = new.convert(&perm);
            adjusted.same(&correct)
        })
        .nth(0)
        .unwrap();

    new.order_map(&correct, &mapping)
}

#[test]
pub fn day_8() {
    if let Ok(lines) = read_lines("./src/year2021/data/day8input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut counts = [0; 10];
        let mut sum = 0;
        for (line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                let x: Vec<_> = contents.split(" | ").collect();
                let signals: Vec<_> = x[0].split(" ").collect();
                let map = map_signals(&signals);

                let output: usize = x[1]
                    .split(" ")
                    .map(|s| {
                        let mut sigs: [bool; 7] = Default::default();
                        str_to_sigs(s, &mut sigs);
                        let pos = map.dat.iter().position(|&sig| sig == sigs).unwrap();
                        counts[pos] += 1;
                        pos
                    })
                    .enumerate()
                    .map(|(pos, val)| val * (10 as usize).pow(3 - pos as u32))
                    .sum();
                sum += output;
            }
        }
        dbg!(sum);
        dbg!(counts);
        dbg!(counts[1] + counts[4] + counts[7] + counts[8]);
    }
}
