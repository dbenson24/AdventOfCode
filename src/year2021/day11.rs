use std::collections::{HashMap, HashSet};

use crate::utils::*;

pub fn flash_octopuses(world: &mut World<i32>) -> i32 {
    world
        .world
        .iter_mut()
        .for_each(|(_pos, octopus)| *octopus += 1);

    let mut stack: Vec<_> = world
        .world
        .iter()
        .filter(|(&_pos, &octopus)| octopus >= 9)
        .map(|x| *x.0)
        .collect();
    let mut flashes = 0;
    while let Some(curr_pos) = stack.pop() {
        if let Some(&octopus) = world.world.get(&curr_pos) {
            if octopus > 9 {
                flashes += 1;
                let neighbors = get_neighbors(curr_pos);
                for neighbor in neighbors {
                    if let Some(neigh_octo) = world.world.get(&neighbor) {
                        let neigh_octo = *neigh_octo + 1;
                        if neigh_octo > 9 {
                            stack.push(neighbor);
                        }
                        if neigh_octo > 1 {
                            world.world.insert(neighbor, neigh_octo);
                        }
                    }
                }
                world.world.insert(curr_pos, 0);
            }
        }
    }
    flashes
}

#[test]
pub fn day_11() {
    if let Some(mut map) = World::<i32>::from_file("./src/year2021/data/day11input.txt", |s| {
        let x: i32 = s.parse().unwrap();
        x
    }) {
        //map.pretty_print();
        let mut total_flashes = 0;
        for day in 1..=500 {
            let flashes = flash_octopuses(&mut map);
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
