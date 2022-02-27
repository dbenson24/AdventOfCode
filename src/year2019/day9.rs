use super::intcode::IntcodeMachine;
use crate::utils::*;
use std::sync::mpsc;
use std::time::Duration;
#[test]
pub fn day_9() {
    if let Ok(lines) = read_lines("./src/year2019/data/day9input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for (_line_num, line) in lines.enumerate() {
            let mut numbers: Vec<i64> = line
                .unwrap()
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect();

            let mut machine = IntcodeMachine::new(numbers);
            let (tx, rx) = mpsc::channel();
            tx.send(Some(2)).unwrap();
            machine.run(&rx, &tx);
            while let Ok(x) = rx.recv_timeout(Duration::new(0, 100000)) {
                dbg!(x);
            }
        }
    }
}
