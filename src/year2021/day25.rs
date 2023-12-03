use crate::utils::*;
use core::cmp::Ordering;
use hashbrown::{HashMap, HashSet};
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Debug)]
pub struct Cucumber {
    pub dir: Dir2,
}

#[test]
pub fn day_25() {
    if let Some(mut map) = World::<Cucumber>::from_file("./src/year2021/data/day25input.txt", |s| {
        let cuc = match s {
            "v" => Some(Cucumber { dir: Dir2::Down }),
            ">" => Some(Cucumber { dir: Dir2::Right }),
            _ => None,
        };
        cuc
    }) {
        let max_x = map.max_x() + 1;
        let max_y = map.max_y() + 1;
        let mut i = 0;
        let mut done = false;
        while !done {
            i += 1;
            done = true;
            let mut new_map = World {
                world: HashMap::new(),
            };

            for (pos, cuc) in map.world.iter().filter(|(_, cuc)| cuc.dir == Dir2::Right) {
                let new_x = (pos.x + 1) % max_x;
                let new_pos = IVec2::new(new_x, pos.y);
                if !map.world.contains_key(&new_pos) {
                    done = false;
                    new_map.world.insert(new_pos, *cuc);
                } else {
                    new_map.world.insert(*pos, *cuc);
                }
            }

            for (pos, cuc) in map.world.iter().filter(|(_, cuc)| cuc.dir == Dir2::Down) {
                let new_y = (pos.y + 1) % max_y;
                let new_pos = IVec2::new(pos.x, new_y);
                if let Some(other_cuc) = map.world.get(&new_pos) {
                    if other_cuc.dir == Dir2::Down {
                        new_map.world.insert(*pos, *cuc);
                        continue;
                    }
                }
                if let Some(other_cuc) = new_map.world.get(&new_pos) {
                    if other_cuc.dir == Dir2::Right {
                        new_map.world.insert(*pos, *cuc);
                        continue;
                    }
                }
                done = false;
                new_map.world.insert(new_pos, *cuc);
            }
            map = new_map;
            dbg!(i);
            // map.pretty_print(&|x| {
            //         match x.dir {
            //             Dir2::Right => ">",
            //             Dir2::Down => "v",
            //             _ => " "
            //         }.to_string()
            //     }, false);
            // if i > 3 {
            //     break
            // }
        }
    }
}
