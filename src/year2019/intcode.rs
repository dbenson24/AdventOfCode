use std::sync::mpsc::{Receiver, Sender};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(FromPrimitive)]
enum Codes {
    ADD = 1,
    MUL = 2,
    INPUT = 3,
    OUTPUT = 4,
    JUMPNONZERO = 5,
    JUMPZERO = 6,
    LESSTHAN = 7,
    EQUALS = 8,
    END = 99,
}

#[derive(Debug, Clone)]
pub struct IntcodeMachine {
    pub numbers: Vec<i32>,
    pub last_output: Option<i32>,
}

impl IntcodeMachine {
    pub fn new(numbers: Vec<i32>) -> Self {
        IntcodeMachine {
            numbers,
            last_output: None,
        }
    }

    pub fn run(&mut self, input: &Receiver<i32>, output: &Sender<i32>) {
        let mut pos = 0;
        //let mut output = None;
        self.last_output = None;
        let mut curr_input = 0;
        loop {
            let operation = self.numbers[pos];
            let op = operation % 100;
            let mode1 = (operation / 100) % 10 == 1;
            let mode2 = (operation / 1000) % 10 == 1;
            let mode3 = (operation / 10000) % 10 == 1;
            let get_val = |i: i32, mode: bool| -> i32 {
                if mode {
                    i
                } else {
                    self.numbers[i as usize]
                }
            };
            match FromPrimitive::from_i32(op) {
                Some(Codes::ADD) => {
                    let a = self.numbers[pos + 1];
                    let b = self.numbers[pos + 2];
                    let dest = self.numbers[pos + 3];
                    //dbg!(a,b,a+b, dest);
                    self.numbers[dest as usize] = get_val(a, mode1) + get_val(b, mode2);
                    pos += 4;
                }
                Some(Codes::MUL) => {
                    let a = self.numbers[pos + 1];
                    let b = self.numbers[pos + 2];
                    let dest = self.numbers[pos + 3];
                    //dbg!(a,b,a*b, dest);
                    self.numbers[dest as usize] = get_val(a, mode1) * get_val(b, mode2);
                    pos += 4;
                }
                Some(Codes::INPUT) => {
                    let dest = self.numbers[pos + 1];
                    self.numbers[dest as usize] = input.recv().unwrap();
                    curr_input += 1;
                    pos += 2;
                }
                Some(Codes::OUTPUT) => {
                    let src = self.numbers[pos + 1];
                    let val = get_val(src, mode1);
                    self.last_output = Some(val);
                    self.last_output;
                    output.send(val).unwrap();
                    pos += 2;
                }
                Some(Codes::JUMPNONZERO) => {
                    let a = self.numbers[pos + 1];
                    let b = self.numbers[pos + 2];
                    if get_val(a, mode1) != 0 {
                        pos = get_val(b, mode2) as usize;
                    } else {
                        pos += 3;
                    }
                }
                Some(Codes::JUMPZERO) => {
                    let a = self.numbers[pos + 1];
                    let b = self.numbers[pos + 2];
                    if get_val(a, mode1) == 0 {
                        pos = get_val(b, mode2) as usize;
                    } else {
                        pos += 3;
                    }
                }
                Some(Codes::LESSTHAN) => {
                    let a = self.numbers[pos + 1];
                    let b = self.numbers[pos + 2];
                    let c = self.numbers[pos + 3];
                    if get_val(a, mode1) < get_val(b, mode2) {
                        self.numbers[c as usize] = 1
                    } else {
                        self.numbers[c as usize] = 0
                    }
                    pos += 4;
                }
                Some(Codes::EQUALS) => {
                    let a = self.numbers[pos + 1];
                    let b = self.numbers[pos + 2];
                    let c = self.numbers[pos + 3];
                    if get_val(a, mode1) == get_val(b, mode2) {
                        self.numbers[c as usize] = 1
                    } else {
                        self.numbers[c as usize] = 0
                    }
                    pos += 4;
                }
                Some(Codes::END) => {
                    return;
                }
                _ => {
                    panic!("hit bad opcode {} at {}", op, pos);
                }
            }
        }
    }
}
