use super::intcode::IntcodeMachine;
use crate::utils::*;
use std::sync::mpsc;

#[test]
pub fn run_opcodes() {
    if let Ok(lines) = read_lines("./src/year2019/data/day5input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for (_line_num, line) in lines.enumerate() {
            let mut numbers: Vec<i64> = line
                .unwrap()
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect();

            let mut machine = IntcodeMachine::new(numbers);
            let (tx, rx) = mpsc::channel();
            tx.send(Some(5)).unwrap();
            machine.run(&rx, &tx);
            dbg!(rx.recv().unwrap());
        }
    }
}
