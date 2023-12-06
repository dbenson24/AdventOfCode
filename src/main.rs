#![allow(unused_must_use, unused_imports, unused_mut, dead_code)]

use std::time::Instant;

mod year2017;
mod year2019;
mod year2021;
mod year2023;

mod utils;

use utils::*;

use crate::utils::puzzle::PuzzleFns;

fn main() {
    let _days = 256;
    //year2021::day6::calc_fish_buckets(days);
    let now = Instant::now();

    year2023::day5::AoC2023Day5::run_tests().unwrap();
    let res = year2023::day5::AoC2023Day5::try_submit().unwrap();
    eprintln!("{res:?}");
    let elapsed = now.elapsed();
    println!("{} ms", elapsed.as_millis());
    //year2021::day6::calc_fish_recurse(days);
    //year2021::day6::calc_fish_iter(days);
}
