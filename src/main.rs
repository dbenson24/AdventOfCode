use std::time::Instant;

mod year2021;

mod utils;
mod year2019;

fn main() {
    let days = 256;
    //year2021::day6::calc_fish_buckets(days);
    let now = Instant::now();

    year2021::day14::day_14_part_2();

    let elapsed = now.elapsed();
    println!("{} ms", elapsed.as_millis());
    //year2021::day6::calc_fish_recurse(days);
    //year2021::day6::calc_fish_iter(days);
}
