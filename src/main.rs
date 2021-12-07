mod year2021;

mod utils;
mod year2019;

fn main() {
    println!("Hello, world!");
    let days = 256;
    year2021::day6::calc_fish_buckets(days);
    //year2021::day6::calc_fish_recurse(days);
    //year2021::day6::calc_fish_iter(days);
}
