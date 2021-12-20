use std::sync::mpsc::channel;

use crate::utils::*;
use crate::year2019::intcode::*;
use hashbrown::{HashMap, HashSet};
use rayon::spawn;

#[test]
pub fn base() {
    let numbers = IntcodeMachine::parse_file("./src/year2019/data/day11input.txt").unwrap();
    let mut robot = IntcodeMachine::new(numbers);
    let (robot_input, rx) = channel();
    let (tx, robot_output) = channel();
    spawn(move || {
        robot.run(&rx, &tx);
        tx.send(None);
    });
    let mut white_panels = HashSet::new();
    let mut painted_panels = HashSet::new();
    let mut pos = IVec2::ZERO;
    let mut dir = Dir2::Up;
    robot_input.send(Some(1));
    loop {
        let color = match robot_output.recv().unwrap() {
            Some(x) => x,
            None => break,
        };
        let turn = match robot_output.recv().unwrap() {
            Some(x) => x,
            None => break,
        };

        if color == 0 && white_panels.contains(&pos) {
            white_panels.remove(&pos);
        }
        if color == 1 {
            white_panels.insert(pos);
        }
        painted_panels.insert(pos);

        dir = if turn == 0 {
            dir.turn_left()
        } else {
            dir.turn_right()
        };

        let diff: IVec2 = dir.into();
        pos = pos + diff;

        if white_panels.contains(&pos) {
            robot_input.send(Some(1));
        } else {
            robot_input.send(Some(0));
        }
    }

    dbg!(painted_panels.len());

    pretty_print_set(&white_panels, &|_| "#".to_string(), 1)
}
