use std::collections::{BTreeSet, HashSet};

use crate::utils::*;

#[test]
pub fn count_intersections() {
    if let Ok(lines) = read_lines("./src/year2021/data/day5input.txt") {
        let mut world: VecWorld<bool> = VecWorld::new();
        let mut intersections: HashSet<IVec2> = HashSet::new();
        let _skip_diagonal = true;
        // Consumes the iterator, returns an (Optional) String
        for (_line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                let points: Vec<_> = contents
                    .split(" -> ")
                    .map(|s| {
                        let nums: Vec<i32> = s.split(",").map(|x| x.parse().unwrap()).collect();
                        IVec2::new(nums[0], nums[1])
                    })
                    .collect();
                let a = points[0];
                let b = points[1];
                let mut flat = false;
                if a.x != b.x && a.y != b.y {
                    if (a.x - b.x).abs() != (a.y - b.y).abs() {
                        continue;
                    }
                } else {
                    flat = true;
                }
                let mut check_pos = |pos| {
                    if *world.get(pos) {
                        intersections.insert(pos);
                    } else {
                        world.set(pos, true);
                    }
                };
                for x in a.x.min(b.x)..=a.x.max(b.x) {
                    for y in a.y.min(b.y)..=a.y.max(b.y) {
                        let x_dist = (x - a.x).abs();
                        let y_dist = (y - a.y).abs();
                        if x_dist == y_dist || (flat && (x_dist == 0 || y_dist == 0)) {
                            let pos = IVec2::new(x, y);
                            check_pos(pos);
                        }
                    }
                }
            }
        }
        dbg!(intersections.len());
    }
}
