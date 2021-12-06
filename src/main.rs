

mod year2021;

mod year2019;
mod utils;

fn main() {
    println!("Hello, world!");
    let days= 180;
    year2021::day6::calc_fish_buckets(256);
    year2021::day6::calc_fish_recurse(256);
    //year2021::day6::calc_fish_iter(200);
}
