use std::collections::{BTreeSet, HashSet};
use std::time::{Duration, Instant};

use crate::utils::*;
use debug_plotter;
use rayon::prelude::*;

pub fn calc_fish_iter(days: usize) {
    if let Ok(lines) = read_lines("./src/year2021/data/day6input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut fish: Vec<i32> = vec![];
        for (_line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                contents
                    .split(",")
                    .map(|s| s.parse().unwrap())
                    .for_each(|x| fish.push(x));
            }
        }

        let start = Instant::now();
        for _day_num in 1..=days {
            let now = Instant::now();
            let new_fish: i32 = fish
                .par_iter_mut()
                .map(|f| {
                    if *f == 0 {
                        *f = 6;
                        1
                    } else {
                        *f -= 1;
                        0
                    }
                })
                .sum();
            let time_summing = now.elapsed().as_millis();
            fish.resize_with(fish.len() + (new_fish as usize), || 8);
            let _time_resizing = now.elapsed().as_millis() - time_summing;
            //dbg!(&fish);
            //dbg!(day_num, time_summing, time_resizing, fish.len());
        }
        println!("day {}: {}", days, fish.len());
        println!(
            "Iter spent {} seconds calculating",
            start.elapsed().as_secs_f64()
        );
    }
}

pub fn fish_helper(fish: &mut Vec<i32>, curr_day: usize, last_day: usize) -> usize {
    if curr_day > last_day {
        return fish.len();
    }
    let new_fish: i32 = fish
        .iter_mut()
        .map(|f| {
            if *f == 0 {
                *f = 6;
                1
            } else {
                *f -= 1;
                0
            }
        })
        .sum();
    fish.resize_with(fish.len() + (new_fish as usize), || 8);
    if curr_day < 6 {
        let (left, right) = fish.split_at(fish.len() / 2);
        let mut left = left.to_owned();
        let mut right = right.to_owned();
        let (a, b) = rayon::join(
            || fish_helper(&mut left, curr_day + 1, last_day),
            || fish_helper(&mut right, curr_day + 1, last_day),
        );
        a + b
    } else {
        fish_helper(fish, curr_day + 1, last_day)
    }
}

pub fn calc_fish_recurse(days: usize) {
    if let Ok(lines) = read_lines("./src/year2021/data/day6input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut fish: Vec<i32> = vec![];

        for (_line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                contents
                    .split(",")
                    .map(|s| s.parse().unwrap())
                    .for_each(|x| fish.push(x));
            }
        }

        let start = Instant::now();
        let fish_count = fish_helper(&mut fish, 1, days);
        println!("day {}: {}", days, fish_count);
        println!(
            "Recursive spent {} seconds calculating",
            start.elapsed().as_secs_f64()
        );
    }
}

pub fn calc_fish_buckets(days: usize) {
    let now = Instant::now();

    if let Ok(lines) = read_lines("./src/year2021/data/day6input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut fish = [0 as i64; 9];
        for (_line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                contents
                    .split(",")
                    .map(|s| s.parse().unwrap())
                    .for_each(|x: usize| fish[x] += 1);
            }
        }

        for _day_num in 1..=days {
            let new_fish = fish[0];
            for i in 0..(fish.len() - 1) {
                fish[i] = fish[i + 1];
            }
            fish[6] += new_fish;
            fish[8] = new_fish;
            let _fish_count = fish.iter().sum::<i64>();
            //dbg!(day_num, fish_count);
            //dbg!(&fish);
            //debug_plotter::plot!(fish_count);
        }

        let fish_count = fish.iter().sum::<i64>();
        let elapsed = now.elapsed().as_micros();

        println!("Buckets counted {}", fish_count);
        println!("{} us", elapsed);
    }
}

#[test]
pub fn test_calc_fish() {
    calc_fish_buckets(200);
}
