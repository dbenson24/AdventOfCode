use crate::utils::*;
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

pub fn eval_int_code(numbers: &mut Vec<i32>, input: i32) -> i32 {
    let mut pos = 0;
    let mut output = 0;
    loop {
        let operation = numbers[pos];
        let op = operation % 100;
        let mode1 = (operation / 100) % 10 == 1;
        let mode2 = (operation / 1000) % 10 == 1;
        let mode3 = (operation / 10000) % 10 == 1;
        let get_val = |i: i32, mode: bool| -> i32 {
            if mode {
                i
            } else {
                numbers[i as usize]
            }
        };
        match FromPrimitive::from_i32(op) {
            Some(Codes::ADD) => {
                let a = numbers[pos + 1];
                let b = numbers[pos + 2];
                let dest = numbers[pos + 3];
                //dbg!(a,b,a+b, dest);
                numbers[dest as usize] = get_val(a, mode1) + get_val(b, mode2);
                pos += 4;
            }
            Some(Codes::MUL) => {
                let a = numbers[pos + 1];
                let b = numbers[pos + 2];
                let dest = numbers[pos + 3];
                //dbg!(a,b,a*b, dest);
                numbers[dest as usize] = get_val(a, mode1) * get_val(b, mode2);
                pos += 4;
            }
            Some(Codes::INPUT) => {
                let dest = numbers[pos + 1];
                numbers[dest as usize] = input;
                pos += 2;
            }
            Some(Codes::OUTPUT) => {
                let src = numbers[pos + 1];
                output = get_val(src, mode1);
                pos += 2;
            }
            Some(Codes::JUMPNONZERO) => {
                let a = numbers[pos + 1];
                let b = numbers[pos + 2];
                if get_val(a, mode1) != 0 {
                    pos = get_val(b, mode2) as usize;
                } else {
                    pos += 3;
                }
            }
            Some(Codes::JUMPZERO) => {
                let a = numbers[pos + 1];
                let b = numbers[pos + 2];
                if get_val(a, mode1) == 0 {
                    pos = get_val(b, mode2) as usize;
                } else {
                    pos += 3;
                }
            }
            Some(Codes::LESSTHAN) => {
                let a = numbers[pos + 1];
                let b = numbers[pos + 2];
                let c = numbers[pos + 3];
                if get_val(a, mode1) < get_val(b, mode2) {
                    numbers[c as usize] = 1
                } else {
                    numbers[c as usize] = 0
                }
                pos += 4;
            }
            Some(Codes::EQUALS) => {
                let a = numbers[pos + 1];
                let b = numbers[pos + 2];
                let c = numbers[pos + 3];
                if get_val(a, mode1) == get_val(b, mode2) {
                    numbers[c as usize] = 1
                } else {
                    numbers[c as usize] = 0
                }
                pos += 4;
            }
            Some(Codes::END) => {
                dbg!(&numbers, output);
                return output;
            }
            _ => {
                panic!("hit bad opcode {} at {}", op, pos);
            }
        }
    }
}

#[test]
pub fn run_opcodes() {
    if let Ok(lines) = read_lines("./src/year2019/data/day5input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for (line_num, line) in lines.enumerate() {
            let mut numbers: Vec<i32> = line
                .unwrap()
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect();
            let res = eval_int_code(&mut numbers, 5);
        }
    }
}
