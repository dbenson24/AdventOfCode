use crate::utils::*;
use hashbrown::{HashMap, HashSet};
use image::ImageBuffer;
use itertools::Itertools;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use rayon::iter::*;

#[test]
pub fn day22_part1() {
    let mut enabled_blocks: HashSet<IVec3> = HashSet::new();

    if let Ok(lines) = read_lines("./src/year2021/data/day22testinput.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut operations = vec![];
        for (_line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                let mut iter = contents.split(" ");
                let on = iter.next().unwrap();
                let on = if on == "on" { true } else { false };
                let positions = iter.next().unwrap();
                let pos = positions
                    .split(",")
                    .map(|s| {
                        let mut iter = s.split("=");
                        let s = iter.nth(1).unwrap();
                        let mut nums = s.split("..");
                        let min: i32 = nums.next().unwrap().parse().unwrap();
                        let max: i32 = nums.next().unwrap().parse().unwrap();
                        (min, max)
                    })
                    .collect_vec();
                operations.push((on, pos));
            }
        }

        for (enabled, region) in operations {
            for x in region[0].0..=region[0].1 {
                if x.abs() > 50 {
                    continue;
                }
                for y in region[1].0..=region[1].1 {
                    if y.abs() > 50 {
                        continue;
                    }
                    for z in region[2].0..=region[2].1 {
                        if z.abs() > 50 {
                            continue;
                        }
                        let pos = IVec3::new(x, y, z);
                        if enabled {
                            enabled_blocks.insert(pos);
                        } else {
                            enabled_blocks.remove(&pos);
                        }
                    }
                }
            }
            dbg!(enabled_blocks.len());
        }
        dbg!(enabled_blocks.len());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AABB {
    pub min: IVec3,
    pub max: IVec3,
}

impl AABB {
    pub fn new(min: IVec3, max: IVec3) -> Self {
        Self { min, max }
    }

    pub fn intersects_aabb(&self, aabb: &AABB) -> bool {
        !(self.max.x < aabb.min.x)
            && !(self.min.x > aabb.max.x)
            && (!(self.max.y < aabb.min.y) && !(self.min.y > aabb.max.y))
            && (!(self.max.z < aabb.min.z) && !(self.min.z > aabb.max.z))
    }

    pub fn contains(&self, aabb: &AABB) -> bool {
        self.max.x > aabb.max.x
            && self.min.x < aabb.min.x
            && self.max.y > aabb.max.y
            && self.min.y < aabb.min.y
            && self.max.z > aabb.max.z
            && self.min.z < aabb.min.z
    }

    pub fn size(&self) -> IVec3 {
        self.max - self.min
    }

    pub fn volume(&self) -> i64 {
        let size = self.size();
        size.x as i64 * size.y as i64 * size.z as i64
    }

    pub fn overlap(&self, aabb: &AABB) -> Option<AABB> {
        let min = self.min.max(aabb.min);
        let max = self.max.min(aabb.max);
        let overlap = AABB::new(min, max);
        if overlap.invalid() {
            None
        } else {
            Some(overlap)
        }
    }

    pub fn invalid(&self) -> bool {
        self.min.x >= self.max.x || self.min.y >= self.max.y || self.min.z >= self.max.z
    }

    pub fn split_aabb(&self, overlap: &AABB) -> SmallVec<[AABB; 20]> {
        let mut split = SmallVec::new();
        let x_pts = [self.min.x, overlap.min.x, overlap.max.x, self.max.x];
        let y_pts = [self.min.y, overlap.min.y, overlap.max.y, self.max.y];
        let z_pts = [self.min.z, overlap.min.z, overlap.max.z, self.max.z];

        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    let min = IVec3::new(x_pts[x], y_pts[y], z_pts[z]);
                    let max = IVec3::new(x_pts[x + 1], y_pts[y + 1], z_pts[z + 1]);
                    let split_aabb = AABB::new(min, max);
                    if !split_aabb.invalid() && &split_aabb != overlap {
                        split.push(split_aabb);
                    }
                }
            }
        }

        split
    }
}

#[test]
pub fn test_overlap() {
    let a = AABB::new(IVec3::ZERO, IVec3::ONE * 10);
    let b = AABB::new(IVec3::ONE, IVec3::ONE * 3);
    dbg!(a.overlap(&b));
}

#[test]
pub fn test_overlap_2() {
    let a = AABB::new(IVec3::ZERO, IVec3::ONE * 10);
    let b = AABB::new(IVec3::new(-5, 2, -20), IVec3::ONE * 3);
    dbg!(a.overlap(&b));
}

pub fn day22_part2() {
    if let Ok(lines) = read_lines("./src/year2021/data/day22input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut operations = vec![];
        for (_line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                let mut iter = contents.split(" ");
                let on = iter.next().unwrap();
                let on = if on == "on" { true } else { false };
                let positions = iter.next().unwrap();
                let pos = positions
                    .split(",")
                    .map(|s| {
                        let mut iter = s.split("=");
                        let s = iter.nth(1).unwrap();
                        let mut nums = s.split("..");
                        let min: i32 = nums.next().unwrap().parse().unwrap();
                        let max: i32 = nums.next().unwrap().parse().unwrap();
                        (min, max + 1)
                    })
                    .collect_vec();
                operations.push((on, pos));
            }
        }

        let mut regions = operations
            .into_iter()
            .map(|(on, r)| {
                let min = IVec3::new(r[0].0, r[1].0, r[2].0);
                let max = IVec3::new(r[0].1, r[1].1, r[2].1);
                (on, AABB::new(min, max))
            })
            .collect_vec();

        let mut enabled_boxes = vec![];
        let mut next_enabled = vec![];

        let mut _region = 0;
        for (enabled, aabb) in regions {
            let mut active_boxes = vec![(enabled, aabb)];
            while let Some((enabled, aabb)) = active_boxes.pop() {
                let mut box_ok = true;
                let mut i = 0;
                while i < enabled_boxes.len() {
                    let on_aabb = enabled_boxes[i];
                    if enabled && aabb.contains(&on_aabb) {
                        enabled_boxes.swap_remove(i);
                        continue;
                    }

                    if enabled && on_aabb.contains(&aabb) {
                        box_ok = false;
                        //enabled_boxes.swap_remove(i);
                        break;
                    }

                    if let Some(overlap) = on_aabb.overlap(&aabb) {
                        enabled_boxes.swap_remove(i);
                        if enabled {
                            active_boxes.push((true, overlap));
                            let mut to_add = aabb.split_aabb(&overlap);
                            //to_add.retain(|x| x.overlap(&on_aabb).is_none());

                            for x in to_add {
                                active_boxes.push((true, x));
                            }
                            box_ok = false;
                        }

                        let mut still_on = on_aabb.split_aabb(&overlap);
                        for x in still_on {
                            active_boxes.push((true, x));
                        }
                        if !box_ok {
                            break;
                        }
                    } else {
                        i += 1;
                    }
                }
                if enabled && box_ok {
                    enabled_boxes.push(aabb);
                }
            }
            _region += 1;
            //dbg!(enabled_boxes.len(), next_enabled.len());

            enabled_boxes.append(&mut next_enabled);

            //dbg!(region, enabled_boxes.len());
        }
        dbg!(enabled_boxes.len());
        /*
        let mut overlap_vol = 0;
        // purely to double check there's no overlapping boxes
        for (i, a) in enabled_boxes.iter().enumerate() {
            for (j, b) in enabled_boxes.iter().enumerate() {
                if i < j {
                    if let Some(overlap) = a.overlap(b) {
                        dbg!(i, a, j, b);
                        overlap_vol += overlap.volume();
                    }
                }
            }
        }
        */

        let volume: i64 = enabled_boxes.iter().map(AABB::volume).sum();
        dbg!(volume);
        // dbg!(overlap_vol);

        //let mut itersects = vec![];
    }
}
