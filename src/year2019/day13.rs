use std::sync::mpsc::channel;

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
    Block,
    Paddle,
    Ball,
}

#[test]
pub fn base() {
    let numbers = IntcodeMachine::parse_file("./src/year2019/data/day13input.txt").unwrap();
    let (game_input, game_output) = IntcodeMachine::spawn(numbers);

    let mut world: World<Tile> = World {
        world: HashMap::new(),
    };
    let get_tile = || {
        let x = game_output.recv().unwrap().unwrap();
        let y = game_output.recv().unwrap().unwrap();
        let tile = game_output.recv().unwrap().unwrap();
        (IVec2::new(x as i32, y as i32), tile)
    };
    let _print_tiles = |tile: &Tile| {
        match tile {
            &Tile::Ball => "O",
            &Tile::Block => "X",
            &Tile::Paddle => "=",
            &Tile::Wall => "#",
            &Tile::Empty => " ",
        }
        .to_string()
    };
    let mut _score = 0;
    let mut paddle_pos = IVec2::ZERO;
    let mut _ball_pos = IVec2::ZERO;
    loop {
        let (pos, val) = get_tile();
        if pos.x == -1 {
            _score = val;
            println!("Score = {}", _score);
        } else {
            let tile: Tile = FromPrimitive::from_i64(val).unwrap();
            world.world.insert(pos, tile);

            match tile {
                Tile::Ball => {
                    _ball_pos = pos;
                    let diff = (_ball_pos - paddle_pos).x;
                    if diff == 0 {
                        game_input.send(Some(0));
                    } else if diff > 0 {
                        game_input.send(Some(1));
                    } else {
                        game_input.send(Some(-1));
                    }
                }
                Tile::Paddle => {
                    paddle_pos = pos;
                }
                _ => (),
            }
        }
        //world.pretty_print(&print_tiles);
    }
}
