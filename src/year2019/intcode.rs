use crate::utils::*;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::{
    error, fmt,
    fmt::Error,
    num::ParseIntError,
    sync::mpsc::{channel, Receiver, Sender},
};

#[derive(FromPrimitive, Debug, Clone, Copy)]
enum Codes {
    ADD = 1,
    MUL = 2,
    INPUT = 3,
    OUTPUT = 4,
    JUMPNONZERO = 5,
    JUMPZERO = 6,
    LESSTHAN = 7,
    EQUALS = 8,
    RELATIVEOFFSET = 9,
    END = 99,
}

#[derive(FromPrimitive, Debug, Clone, Copy)]
enum ParamMode {
    POSITION = 0,
    IMMEDIATE = 1,
    RELATIVE = 2,
}

#[derive(Debug, Clone)]
pub struct IntcodeMachine {
    pub numbers: Vec<i64>,
    pub last_output: Option<i64>,
}

impl IntcodeMachine {
    pub fn new(mut numbers: Vec<i64>) -> Self {
        numbers.resize(numbers.len() + 1000, 0);
        IntcodeMachine {
            numbers,
            last_output: None,
        }
    }

    pub fn parse_file(path: &str) -> Result<Vec<i64>, Box<dyn error::Error>> {
        let lines = read_lines(path)?;
        for (line_num, line) in lines.enumerate() {
            let numbers: Vec<i64> = line?.split(",").map(|s| s.parse().unwrap()).collect();
            return Ok(numbers);
        }

        Err(Box::new(ParseIntVecErr))
    }

    pub fn looped_channels<T>(count: usize) -> Vec<(Receiver<T>, Sender<T>)> {
        let (tx_0, mut rx_prev) = channel();
        let (tx_n, rx_n) = channel();
        let mut chans = vec![];
        for _i in 0..(count - 2) {
            chans.push(channel());
        }

        let mut linked_chans = vec![];
        linked_chans.push((rx_n, tx_0));
        for (tx, rx) in chans {
            linked_chans.push((rx_prev, tx));
            rx_prev = rx;
        }
        linked_chans.push((rx_prev, tx_n));

        linked_chans
    }

    pub fn run(&mut self, input: &Receiver<Option<i64>>, output: &Sender<Option<i64>>) {
        let mut pos = 0;
        //let mut output = None;
        self.last_output = None;
        let mut relative_base: i64 = 0;
        loop {
            let operation = self.numbers[pos];
            let op = operation % 100;
            let mode1 = FromPrimitive::from_i64((operation / 100) % 10).unwrap();
            let mode2 = FromPrimitive::from_i64((operation / 1000) % 10).unwrap();
            let mode3 = FromPrimitive::from_i64((operation / 10000) % 10).unwrap();
            let get_index = |pos: usize, mode: ParamMode| -> usize {
                match mode {
                    ParamMode::IMMEDIATE => pos,
                    ParamMode::POSITION => self.numbers[pos] as usize,
                    ParamMode::RELATIVE => (self.numbers[pos] + relative_base) as usize,
                }
            };
            let param_1 = || get_index(pos + 1, mode1);
            let param_2 = || (get_index(pos + 1, mode1), get_index(pos + 2, mode2));
            let param_3 = || {
                (
                    get_index(pos + 1, mode1),
                    get_index(pos + 2, mode2),
                    get_index(pos + 3, mode3),
                )
            };
            match FromPrimitive::from_i64(op) {
                Some(Codes::ADD) => {
                    let (a, b, c) = param_3();
                    self.numbers[c] = self.numbers[a] + self.numbers[b];
                    pos += 4;
                }
                Some(Codes::MUL) => {
                    let (a, b, c) = param_3();
                    //dbg!(a,b,a*b, dest);
                    self.numbers[c] = self.numbers[a] * self.numbers[b];
                    pos += 4;
                }
                Some(Codes::INPUT) => {
                    let a = param_1();
                    self.numbers[a] = match input.recv().unwrap() {
                        Some(x) => x,
                        None => return,
                    };
                    pos += 2;
                }
                Some(Codes::OUTPUT) => {
                    let a = param_1();
                    let val = self.numbers[a];
                    self.last_output = Some(val);
                    self.last_output;
                    output.send(Some(val)).unwrap();
                    pos += 2;
                }
                Some(Codes::JUMPNONZERO) => {
                    let (a, b) = param_2();
                    if self.numbers[a] != 0 {
                        pos = self.numbers[b] as usize;
                    } else {
                        pos += 3;
                    }
                }
                Some(Codes::JUMPZERO) => {
                    let (a, b) = param_2();
                    if self.numbers[a] == 0 {
                        pos = self.numbers[b] as usize;
                    } else {
                        pos += 3;
                    }
                }
                Some(Codes::LESSTHAN) => {
                    let (a, b, c) = param_3();
                    if self.numbers[a] < self.numbers[b] {
                        self.numbers[c as usize] = 1
                    } else {
                        self.numbers[c as usize] = 0
                    }
                    pos += 4;
                }
                Some(Codes::EQUALS) => {
                    let (a, b, c) = param_3();
                    if self.numbers[a] == self.numbers[b] {
                        self.numbers[c as usize] = 1
                    } else {
                        self.numbers[c as usize] = 0
                    }
                    pos += 4;
                }
                Some(Codes::RELATIVEOFFSET) => {
                    let a = param_1();
                    relative_base += self.numbers[a];
                    pos += 2;
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

#[derive(Debug)]
struct ParseIntVecErr;

impl fmt::Display for ParseIntVecErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unable to parse the vec of ints")
    }
}

impl error::Error for ParseIntVecErr {}
