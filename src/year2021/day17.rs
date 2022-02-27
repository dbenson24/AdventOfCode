use itertools::Itertools;

use crate::utils::*;

pub fn find_highest(min: IVec2, max: IVec2) -> (i32, usize) {
    let mut highest_y = 0;
    let mut hits = 0;
    for start_vx in -500..500 {
        for start_vy in -5000..5000 {
            let mut vel = IVec2::new(start_vx, start_vy);
            let mut pos = IVec2::new(0, 0);
            let mut hit_target = false;
            let mut curr_high_y = 0;
            loop {
                pos += vel;
                vel.x = (vel.x - 1).max(0);
                vel.y -= 1;

                curr_high_y = curr_high_y.max(pos.y);

                if pos.x >= min.x && pos.x <= max.x && pos.y >= min.y && pos.y <= max.y {
                    hit_target = true;
                }

                if hit_target && vel.y < 0 {
                    break;
                }
                if vel.x == 0 && pos.x < min.x {
                    break;
                }
                if vel.y < 0 && pos.y < min.y {
                    break;
                }
            }
            if hit_target {
                hits += 1;
                highest_y = highest_y.max(curr_high_y);
            }
        }
    }

    (highest_y, hits)
}

#[test]
pub fn test() {
    dbg!(find_highest(IVec2::new(20, -10), IVec2::new(30, -5)));
}

#[test]
pub fn part_1() {
    dbg!(find_highest(IVec2::new(96, -144), IVec2::new(125, -98)));
}

#[test]
pub fn base() {
    if let Ok(lines) = read_lines("./src/year2021/data/day17input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for (_line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                let split: Vec<_> = contents
                    .split(", ")
                    .map(|x| {
                        let range = x.split("=").nth(1).unwrap();

                        let vals: Vec<i32> =
                            range.split("..").map(|s| s.parse().unwrap()).collect();
                        (vals[0], vals[1])
                    })
                    .collect();
                let x_region = split[0];
                let y_region = split[1];

                let min = IVec2::new(x_region.0, y_region.0);
                let max = IVec2::new(x_region.1, y_region.1);
                dbg!(find_highest(min, max));
            }
        }
    }
}
