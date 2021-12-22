#![allow(unused_must_use, unused_imports, unused_mut, dead_code)]

use std::time::Instant;

mod year2021;

mod utils;
mod year2019;

use utils::*;

fn main() {
    let days = 256;
    //year2021::day6::calc_fish_buckets(days);
    let now = Instant::now();
    year2021::day22::day22_part2();
    let elapsed = now.elapsed();
    println!("{} ms", elapsed.as_millis());
    //year2021::day6::calc_fish_recurse(days);
    //year2021::day6::calc_fish_iter(days);
}
