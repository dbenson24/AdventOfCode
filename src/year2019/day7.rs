use std::sync::mpsc;

use itertools::Itertools;
use rayon::iter::*;

use crate::utils::*;

use super::intcode::IntcodeMachine;

fn run_circuit(machine: &IntcodeMachine, inputs: &[i64]) -> i64 {
    let mut signal = 0;

    let linked_chans = IntcodeMachine::looped_channels(inputs.len());

    for i in 0..inputs.len() {
        let chan_i = if i == 0 { inputs.len() - 1 } else { i - 1 };
        linked_chans[chan_i].1.send(Some(inputs[i])).unwrap();
    }
    linked_chans[linked_chans.len() - 1].1.send(Some(0));

    let machines: Vec<_> = linked_chans
        .into_par_iter()
        .map(|(rx, tx)| {
            let mut m = machine.clone();
            m.run(&rx, &tx);
            (m, (rx, tx))
        })
        .collect();

    machines[machines.len() - 1].0.last_output.unwrap()
}

#[test]
pub fn day_7_part_1() {
    if let Ok(lines) = read_lines("./src/year2019/data/day7input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for (line_num, line) in lines.enumerate() {
            let mut numbers: Vec<i64> = line
                .unwrap()
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect();

            let machine = IntcodeMachine::new(numbers);

            let mut highest = 0;
            for input in (0..5).permutations(5) {
                highest = highest.max(run_circuit(&machine, &input));
            }
            dbg!(highest);
        }
    }
}

#[test]
pub fn day_7_part_2() {
    if let Ok(lines) = read_lines("./src/year2019/data/day7input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for (line_num, line) in lines.enumerate() {
            let mut numbers: Vec<i64> = line
                .unwrap()
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect();

            let machine = IntcodeMachine::new(numbers);

            let mut highest = 0;
            for input in (5..10).permutations(5) {
                highest = highest.max(run_circuit(&machine, &input));
            }
            dbg!(highest);
        }
    }
}
