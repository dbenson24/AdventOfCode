use crate::utils::*;
use core::cmp::Ordering;
use hashbrown::{HashMap, HashSet};
use std::collections::BinaryHeap;
use std::num::Wrapping;

pub fn test_digit(digit: i64, z: i64) -> i64 {
    let x = z % 26;
    let x = x + 14;
    let x = x == digit;
    let x = (x == false) as i64;
    let y = (25 * x) + 1;
    let z = z * y;
    let y = digit + 12;
    let y = y * x;
    z + y
}
pub fn test_digit_with_params(digit: i64, z: i64, z_div: i64, x_add: i64, y_add: i64) -> i64 {
    let x = z % 26;
    let z = z / z_div;
    let x = x + x_add;
    let x = x == digit;
    let x = (x == false) as i64;
    let y = (25 * x) + 1;
    let z = z * y;
    let y = digit + y_add;
    let y = y * x;
    z + y
}

pub fn test_ith_digit(digit: i64, z: i64, i: usize) -> i64 {
    const CONSTANTS: [[i64; 14]; 3] = [
        [1, 1, 1, 26, 1, 1, 1, 26, 1, 26, 26, 26, 26, 26],
        [14, 10, 13, -8, 11, 11, 14, -11, 14, -1, -8, -5, -16, -6],
        [12, 9, 8, 3, 0, 11, 10, 13, 3, 10, 10, 14, 6, 5],
    ];
    let z_div = CONSTANTS[0][i];
    let x_add = CONSTANTS[1][i];
    let y_add = CONSTANTS[2][i];

    test_digit_with_params(digit, z, z_div, x_add, y_add)
}

// z only has 26 values
//
pub fn test_digit_2(digit: i64, z: i64) -> i64 {
    let x = (z % 26) + 14;

    let y = if x != digit {
        26
    } else {
        dbg!(digit, z);
        1
    };
    let z = Wrapping(z) * Wrapping(y);

    if x != digit {
        z.0 + digit + 12
    } else {
        z.0
    }
}

pub fn test_digit_3(digit: i64, z: i64) -> i64 {
    let x = (z % 26) + 14;
    let z = z * 26;
    z + digit + 12
}

#[test]
pub fn test_fns() {
    for dig in 1..=9 {
        for z in 0..1000 {
            assert_eq!(test_digit(dig, z), run_original(dig, z));
            assert_eq!(test_digit(dig, z), test_digit_2(dig, z));
            assert_eq!(test_digit(dig, z), test_digit_3(dig, z));
        }
    }
}

#[test]
pub fn test_num() {
    let num = [1, 3, 4, 7, 9, 2, 4, 6, 8, 9, 9, 9, 9, 9];
    let mut z = 0;
    for x in num {
        z = test_digit_2(x, z)
    }
    dbg!(z);
    let mut z = 0;
    for x in num {
        z = run_original(x, z)
    }
    dbg!(z);
}

pub fn run_original(digit: i64, z: i64) -> i64 {
    let mut memory = HashMap::new();
    if let Ok(lines) = read_lines("./src/year2021/data/day24testinput.txt") {
        let input = [digit];
        let mut curr_input = 0usize;
        // Consumes the iterator, returns an (Optional) String
        memory.insert("z".to_string(), z);
        for (line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                let mut statement = contents.split(" ");

                let op = statement.next().unwrap();
                let var1 = statement.next().unwrap();
                let var2 = statement.next();
                let val = |memory: &HashMap<String, i64>| {
                    if let Ok(val) = i64::from_str_radix(var2.unwrap(), 10) {
                        val
                    } else {
                        *memory.get(var2.unwrap()).unwrap_or(&0)
                    }
                };
                match op {
                    "inp" => {
                        memory.insert(var1.to_string(), input[curr_input]);
                        curr_input += 1;
                    }
                    "add" => {
                        memory.insert(
                            var1.to_string(),
                            *memory.get(var1).unwrap_or(&0) + val(&memory),
                        );
                    }
                    "mul" => {
                        memory.insert(
                            var1.to_string(),
                            (Wrapping(*memory.get(var1).unwrap_or(&0)) * Wrapping(val(&memory))).0,
                        );
                    }
                    "div" => {
                        memory.insert(
                            var1.to_string(),
                            *memory.get(var1).unwrap_or(&0) / val(&memory),
                        );
                    }
                    "mod" => {
                        memory.insert(
                            var1.to_string(),
                            *memory.get(var1).unwrap_or(&0) % val(&memory),
                        );
                    }
                    "eql" => {
                        memory.insert(
                            var1.to_string(),
                            (*memory.get(var1).unwrap_or(&0) == val(&memory)) as i64,
                        );
                    }
                    _ => (),
                }
            }
        }
    }
    memory["z"]
}

#[test]
pub fn base() {
    if let Ok(lines) = read_lines("./src/year2021/data/day24input.txt") {
        let input = [1, 3, 4, 7, 9, 2, 4, 6, 8, 9, 9, 9, 9, 9];
        let mut curr_input = 0usize;
        let mut constants = vec![Vec::<i64>::new(); 10];
        let mut curr_const = 0;
        // Consumes the iterator, returns an (Optional) String
        let z = 0;
        let mut memory = HashMap::new();
        memory.insert("z".to_string(), z);
        for (line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                let mut statement = contents.split(" ");

                let op = statement.next().unwrap();
                let var1 = statement.next().unwrap();
                let var2 = statement.next();
                let mut val = |memory: &HashMap<String, i64>| {
                    if let Ok(val) = i64::from_str_radix(var2.unwrap(), 10) {
                        constants[curr_const].push(val);
                        curr_const += 1;
                        val
                    } else {
                        *memory.get(var2.unwrap()).unwrap_or(&0)
                    }
                };
                match op {
                    "inp" => {
                        println!("inp");
                        curr_const = 0;
                        memory.insert(var1.to_string(), input[curr_input]);
                        curr_input += 1;
                    }
                    "add" => {
                        memory.insert(
                            var1.to_string(),
                            *memory.get(var1).unwrap_or(&0) + val(&memory),
                        );
                    }
                    "mul" => {
                        memory.insert(
                            var1.to_string(),
                            *memory.get(var1).unwrap_or(&0) * val(&memory),
                        );
                    }
                    "div" => {
                        memory.insert(
                            var1.to_string(),
                            *memory.get(var1).unwrap_or(&0) / val(&memory),
                        );
                    }
                    "mod" => {
                        memory.insert(
                            var1.to_string(),
                            *memory.get(var1).unwrap_or(&0) % val(&memory),
                        );
                    }
                    "eql" => {
                        memory.insert(
                            var1.to_string(),
                            (*memory.get(var1).unwrap_or(&0) == val(&memory)) as i64,
                        );
                    }
                    _ => (),
                }
                //println!("{}", &contents);
                //dbg!(&memory);
            }
        }
        dbg!(test_digit(input[0], z));
        dbg!(memory);
        let mut test_z = 0;
        for (i, &digit) in input.iter().enumerate() {
            test_z = test_ith_digit(digit, test_z, i)
        }
        dbg!(test_z);
    }
}
