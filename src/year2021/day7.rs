use crate::utils::*;

#[test]
pub fn day7_part1() {
    if let Ok(lines) = read_lines("./src/year2021/data/day7input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for (line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                let positions: Vec<usize> = contents.split(",").map(|s| s.parse().unwrap()).collect();

                let mut min_cost = 99999999;
                for i in 0..positions.len() {
                    let cost: i32 = positions.iter().map(|x| ((*x as i32 - i as i32).abs() * ((*x as i32 - i as i32).abs() + 1)) / 2).sum();
                    min_cost = cost.min(min_cost);
                }
                dbg!(min_cost);
                let avg = (positions.iter().map(|x| *x as f64).sum::<f64>() / positions.len() as f64).round() as usize;
                dbg!(avg);
            }
        }
    }
}

#[test]
pub fn day7_part2() {
    if let Ok(lines) = read_lines("./src/year2021/data/day7testinput.txt") {
        // Consumes the iterator, returns an (Optional) String
        for (line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {

            }
        }
    }
}
