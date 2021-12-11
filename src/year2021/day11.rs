use std::collections::{HashMap, HashSet};

use crate::utils::*;

struct OctopusMap {
    pub world: HashMap<Vec2, i32>,
    pub max_x: i32,
    pub max_y: i32,
}

impl OctopusMap {
    fn get_neighbors(pos: Vec2) -> [Vec2; 8] {
        let x = pos.x;
        let y = pos.y;
        let mut i = 0;
        let mut neighs: [Vec2; 8] = Default::default();
        for x_i in x - 1..x + 2 {
            for y_i in y - 1..y + 2 {
                if !(x == x_i && y == y_i) {
                    neighs[i] = Vec2::new(x_i, y_i);
                    i += 1;
                }
            }
        }
        neighs
    }

    pub fn flash_octopuses(&mut self) -> i32 {
        self.world
            .iter_mut()
            .for_each(|(pos, octopus)| *octopus += 1);

        let mut stack: Vec<_> = self
            .world
            .iter()
            .filter(|(&pos, &octopus)| octopus >= 9)
            .map(|x| *x.0)
            .collect();
        let mut flashes = 0;
        while let Some(curr_pos) = stack.pop() {
            if let Some(&octopus) = self.world.get(&curr_pos) {
                if octopus > 9 {
                    flashes += 1;
                    let neighbors = OctopusMap::get_neighbors(curr_pos);
                    for neighbor in neighbors {
                        if let Some(neigh_octo) = self.world.get(&neighbor) {
                            let neigh_octo = *neigh_octo + 1;
                            if neigh_octo > 9 {
                                stack.push(neighbor);
                            }
                            if neigh_octo > 1 {
                                self.world.insert(neighbor, neigh_octo);
                            }
                        }
                    }
                    self.world.insert(curr_pos, 0);
                }
            }
        }
        flashes
    }

    pub fn pretty_print(&self) {
        for y in 0..self.max_y {
            let mut row = vec![];
            for x in 0..=self.max_x {
                let pos = Vec2::new(x, y);
                row.push(self.world[&pos]);
            }
            println!("{:?}", row);
        }
    }
}

#[test]
pub fn day_11() {
    if let Ok(lines) = read_lines("./src/year2021/data/day11input.txt") {
        let mut world: HashMap<Vec2, i32> = HashMap::new();
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
                    let pos = Vec2::new(x as i32, y as i32);
                    world.insert(pos, height);
                    max_x = max_x.max(x);
                }
            }
            max_y = max_y.max(y);
        }
        let mut map = OctopusMap {
            world,
            max_x: max_x as i32,
            max_y: max_y as i32,
        };

        //map.pretty_print();
        let mut total_flashes = 0;
        for day in 1..=500 {
            let flashes = map.flash_octopuses();
            if flashes == 100 {
                println!("all octopus flashed on day {}", day);
                break;
            }
            total_flashes += flashes;

            if day == 100 {
                println!("There were {} total flashes on day 100", total_flashes);
            }
        }
        dbg!(total_flashes);
    }
}
