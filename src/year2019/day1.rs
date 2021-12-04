use crate::utils::*;


fn get_fuel(x: i32) -> i32 {
    ((x / 3) - 2).max(0)
}

fn get_total_fuel(x: i32) -> i32 {
    let mut total_fuel = get_fuel(x);
    let mut extra_fuel = get_fuel(total_fuel);
    while extra_fuel > 0 {
        total_fuel += extra_fuel;
        extra_fuel = get_fuel(extra_fuel)
    }
    total_fuel
}

#[test]
pub fn calc_fuel() {
    if let Ok(lines_iter) = read_lines("./src/year2019/data/day1input.txt") {
        let lines: Vec<_> = lines_iter.map(|s| s.unwrap()).collect();
        let base_fuel = lines.iter().map(|s| s.parse::<i32>().unwrap()).map(get_fuel).sum::<i32>();
        dbg!(base_fuel);
        let total_fuel = lines.iter().map(|s| s.parse::<i32>().unwrap()).map(get_total_fuel).sum::<i32>();
        dbg!(total_fuel);
    }
}

