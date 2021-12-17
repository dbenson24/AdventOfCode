use std::collections::{HashMap, HashSet};

use crate::utils::*;

struct HeightMap {
    pub world: HashMap<IVec2, i32>,
    pub max_x: i32,
    pub max_y: i32,
}

impl HeightMap {
    pub fn get_low_points(&self) -> Vec<IVec2> {
        let mut lows = vec![];
        for x in 0..=self.max_x as i32 {
            for y in 0..=self.max_y as i32 {
                {
                    let pos = IVec2::new(x, y);
                    if self.is_low_point(pos) {
                        lows.push(pos)
                    }
                }
            }
        }
        lows
    }

    fn is_low_point(&self, pos: IVec2) -> bool {
        let positions = HeightMap::get_neighbors(pos);
        let height = self.world[&pos];
        positions.iter().all(|pos| {
            if let Some(neigh_height) = self.world.get(pos) {
                *neigh_height > height
            } else {
                true
            }
        })
    }

    fn get_neighbors(pos: IVec2) -> [IVec2; 4] {
        let x = pos.x;
        let y = pos.y;
        [
            IVec2::new(x, y - 1),
            IVec2::new(x, y + 1),
            IVec2::new(x - 1, y),
            IVec2::new(x + 1, y),
        ]
    }

    pub fn calc_basin_size(&self, pos: IVec2) -> i32 {
        if !self.is_low_point(pos) {
            panic!("trying to calc basin from non low point");
        }

        let mut stack = vec![pos];
        let mut visited: HashSet<IVec2> = HashSet::new();
        let mut size = 0;
        while let Some(curr_pos) = stack.pop() {
            if visited.contains(&curr_pos) {
                continue;
            }
            visited.insert(curr_pos);
            if let Some(&height) = self.world.get(&curr_pos) {
                if height < 9 {
                    let neighbors = HeightMap::get_neighbors(curr_pos);
                    for neighbor in neighbors {
                        stack.push(neighbor);
                    }
                    size += 1;
                }
            }
        }
        size
    }
}

#[test]
pub fn day_9_part_1() {
    if let Ok(lines) = read_lines("./src/year2021/data/day9input.txt") {
        let mut world: HashMap<IVec2, i32> = HashMap::new();
        // Consumes the iterator, returns an (Optional) String
        let mut max_x = 0;
        let mut max_y = 0;
        for (y, line) in lines.enumerate() {
            if let Ok(contents) = line {
                for (x, height) in contents
                    .split("")
                    .filter(|s| s.len() > 0)
                    .map(|s| s.parse::<i32>().unwrap())
                    .enumerate()
                {
                    let pos = IVec2::new(x as i32, y as i32);
                    world.insert(pos, height);
                    max_x = max_x.max(x);
                }
            }
            max_y = max_y.max(y);
        }
        let map = HeightMap {
            world,
            max_x: max_x as i32,
            max_y: max_y as i32,
        };
        let mut low_count = 0;
        let mut low_sum = 0;
        for low in map.get_low_points() {
            let height = map.world[&low];
            low_count += 1;
            low_sum += 1 + height;
        }
        dbg!(low_count, low_sum);
    }
}

#[test]
pub fn day_9_part_2() {
    if let Ok(lines) = read_lines("./src/year2021/data/day9input.txt") {
        let mut world: HashMap<IVec2, i32> = HashMap::new();
        // Consumes the iterator, returns an (Optional) String
        let mut max_x = 0;
        let mut max_y = 0;
        for (y, line) in lines.enumerate() {
            if let Ok(contents) = line {
                for (x, height) in contents
                    .split("")
                    .filter(|s| s.len() > 0)
                    .map(|s| s.parse::<i32>().unwrap())
                    .enumerate()
                {
                    let pos = IVec2::new(x as i32, y as i32);
                    world.insert(pos, height);
                    max_x = max_x.max(x);
                }
            }
            max_y = max_y.max(y);
        }
        let map = HeightMap {
            world,
            max_x: max_x as i32,
            max_y: max_y as i32,
        };
        let mut sizes: Vec<_> = map
            .get_low_points()
            .iter()
            .map(|pos| map.calc_basin_size(*pos))
            .collect();
        sizes.sort();
        let l = sizes.len();
        dbg!(sizes[l - 1] * sizes[l - 2] * sizes[l - 3]);
    }
}
