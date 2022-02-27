use std::{sync::mpsc::channel, time::Duration};

use crate::utils::*;
use crate::year2019::intcode::*;
use hashbrown::{HashMap, HashSet};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use rayon::spawn;

#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Wall,
    Target,
}

pub fn to_command(dir: &Dir2) -> i64 {
    match dir {
        Dir2::Left => 3,
        Dir2::Up => 1,
        Dir2::Right => 4,
        Dir2::Down => 2,
    }
}

pub fn day_15() {
    let numbers = IntcodeMachine::parse_file("./src/year2019/data/day15input.txt").unwrap();
    let (input, output) = IntcodeMachine::spawn(numbers);

    let mut state: World<Tile> = World {
        world: HashMap::new(),
    };
    let print_tiles = |tile: &Tile| {
        match tile {
            &Tile::Empty => ".",
            &Tile::Wall => "#",
            &Tile::Target => "O",
        }
        .to_string()
    };
    let make_printer = |robot: IVec2| {
        move |tile: &Tile, pos: IVec2| {
            if pos == robot {
                "R".to_string()
            } else {
                print_tiles(tile)
            }
        }
    };
    state.world.insert(IVec2::ZERO, Tile::Empty);

    // visit positions based on walking distance from origin
    // walk robot from curr pos to test pos
    // if test == target, terminate and calc cost
    // if test == wall, continue
    // if test was walkable, queue tests neighbors

    let mut curr_pos = IVec2::ZERO;
    let mut oxygen_start = IVec2::ZERO;

    state.min_cost_4(
        &mut (&mut curr_pos, &mut oxygen_start),
        IVec2::ZERO,
        |next_pos, world, (curr_pos, oxygen_start)| {
            let res = world.min_path_4(
                &mut 0,
                **curr_pos,
                |y, _, _| y == next_pos,
                |p, w, _| {
                    if let Some(tile) = w.world.get(&p) {
                        if tile == &Tile::Wall {
                            return None;
                        } else {
                            Some(1)
                        }
                    } else {
                        if p == next_pos {
                            Some(1)
                        } else {
                            None
                        }
                    }
                },
            );

            if let Some((cost, path)) = res {
                if path.len() > 1 {
                    for pos in &path {
                        if *pos == **curr_pos || *pos == next_pos {
                            continue;
                        }
                        let dir = *pos - **curr_pos;

                        // dbg!(*curr_pos);
                        let cmd = to_command(&dir.into());
                        input.send(Some(cmd)).unwrap();
                        let res = output.recv().unwrap().unwrap();

                        match res {
                            0 => {
                                // world.pretty_print(&make_printer(*curr_pos), true);
                                dbg!(curr_pos, pos, dir, &path, next_pos);
                                panic!("unable to navigate to already found node")
                            }
                            _ => {
                                **curr_pos += dir;
                            }
                        }
                    }
                }

                if **curr_pos != next_pos {
                    let dir = next_pos - **curr_pos;

                    let cmd = to_command(&dir.into());
                    input.send(Some(cmd)).unwrap();
                    let res = output.recv().unwrap().unwrap();
                    if **curr_pos + dir != next_pos {
                        panic!("somehow had bad pos")
                    }

                    match res {
                        0 => {
                            world.world.insert(**curr_pos + dir, Tile::Wall);
                        }
                        1 => {
                            **curr_pos += dir;
                            world.world.insert(**curr_pos, Tile::Empty);
                        }
                        2 => {
                            **curr_pos += dir;
                            world.world.insert(**curr_pos, Tile::Target);
                            **oxygen_start = **curr_pos;
                        }
                        _ => {
                            panic!("bad response from intcode");
                        }
                    }

                    // world.pretty_print(&make_printer(*curr_pos), true);
                }
            }

            false
        },
        |pos, world, state| {
            world.min_cost_4(
                &mut 0,
                IVec2::ZERO,
                |y, _, _| y == pos,
                |p, w, _| {
                    if let Some(tile) = w.world.get(&p) {
                        if (tile == &Tile::Wall) {
                            return None;
                        } else {
                            Some(1)
                        }
                    } else {
                        if p == pos {
                            Some(1)
                        } else {
                            None
                        }
                    }
                },
            )
        },
    );

    state.pretty_print(&make_printer(IVec2::ZERO), true);

    let x = state.min_cost_4(
        &mut 0,
        IVec2::ZERO,
        |y, _, _| y == oxygen_start,
        |p, w, _| {
            if let Some(tile) = w.world.get(&p) {
                if (tile == &Tile::Wall) {
                    return None;
                } else {
                    Some(1)
                }
            } else {
                if p == oxygen_start {
                    Some(1)
                } else {
                    None
                }
            }
        },
    );
    dbg!(x);

    let mut mins = -1;
    let mut updates = HashSet::new();
    updates.insert(oxygen_start);
    while !updates.is_empty() {
        mins += 1;
        let mut next = HashSet::new();

        for pos in updates {
            state.world.insert(pos, Tile::Target);
            for neigh in get_cardinal_neighbors(pos) {
                if let Some(tile) = state.world.get(&neigh) {
                    match tile {
                        Tile::Empty => {
                            next.insert(neigh);
                        }
                        _ => (),
                    }
                }
            }
        }
        updates = next;
    }

    dbg!(mins);
    input.send(None);
}
