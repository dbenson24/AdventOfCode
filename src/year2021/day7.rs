use crate::utils::*;

#[test]
pub fn day7_part1() {
    if let Ok(lines) = read_lines("./src/year2021/data/day7input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for (_line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                let positions: Vec<i32> = contents.split(",").map(|s| s.parse().unwrap()).collect();

                let min_cost = positions
                    .iter()
                    .map(|i| positions.iter().map(|x| (*x - i).abs()).sum::<i32>())
                    .min();

                dbg!(min_cost);
            }
        }
    }
}

#[test]
pub fn day7_part2() {
    if let Ok(lines) = read_lines("./src/year2021/data/day7input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for (_line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                let positions: Vec<i32> = contents.split(",").map(|s| s.parse().unwrap()).collect();

                let min_cost = positions
                    .iter()
                    .map(|i| {
                        positions
                            .iter()
                            .map(|x| (*x - i).abs())
                            .map(|x| (x * (x + 1)) / 2)
                            .sum::<i32>()
                    })
                    .min();

                dbg!(min_cost);
            }
        }
    }
}
