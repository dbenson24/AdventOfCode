use std::time::Instant;

mod year2021;

mod utils;
mod year2019;

use utils::*;

fn main() {
    let days = 256;
    //year2021::day6::calc_fish_buckets(days);
    let now = Instant::now();

    let x = year2021::day17::find_highest(IVec2::new(20, -10), IVec2::new(30, -5));
    dbg!(x);

    let x = year2021::day17::find_highest(IVec2::new(96, -144), IVec2::new(125, -98));
    dbg!(x);
    let elapsed = now.elapsed();
    println!("{} ms", elapsed.as_millis());
    //year2021::day6::calc_fish_recurse(days);
    //year2021::day6::calc_fish_iter(days);
}
