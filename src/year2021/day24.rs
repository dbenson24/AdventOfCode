use crate::{utils::*};
use core::cmp::Ordering;
use hashbrown::{HashMap, HashSet};
use std::collections::BinaryHeap;


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
    let z = z * y;

    if x != digit {
        z + digit + 12
    } else {
        z
    }
}

pub fn test_digit_3(digit: i64, z: i64) -> i64 {
    let x = (z % 26) + 14;
    let z = z * 26;

    if x != digit {
        z + digit + 12
    } else {
        z
    }
}

#[test]
pub fn test_fns() {
    for dig in 1..=9 {
        for z in 0..1000 {
            assert_eq!(test_digit(dig, z), test_digit_2(dig, z));
        }
    }
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
                    },
                    "add" => {
                        memory.insert(var1.to_string(), *memory.get(var1).unwrap_or(&0) + val(&memory));
                    },
                    "mul" => {
                        memory.insert(var1.to_string(), *memory.get(var1).unwrap_or(&0) * val(&memory));

                    },
                    "div" => {
                        memory.insert(var1.to_string(), *memory.get(var1).unwrap_or(&0) / val(&memory));
                    },
                    "mod" => {
                        memory.insert(var1.to_string(), *memory.get(var1).unwrap_or(&0) % val(&memory));
                    },
                    "eql" => {
                        memory.insert(var1.to_string(), (*memory.get(var1).unwrap_or(&0) == val(&memory)) as i64);
                    },
                    _ => ()
                }

            }
        }
    }
    memory["z"]
}

#[test]
pub fn base() {
    if let Ok(lines) = read_lines("./src/year2021/data/day24testinput.txt") {
        let input = [5];
        let mut curr_input = 0usize;
        // Consumes the iterator, returns an (Optional) String
        let z = 15;
        let mut memory = HashMap::new();
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
                    },
                    "add" => {
                        memory.insert(var1.to_string(), *memory.get(var1).unwrap_or(&0) + val(&memory));
                    },
                    "mul" => {
                        memory.insert(var1.to_string(), *memory.get(var1).unwrap_or(&0) * val(&memory));

                    },
                    "div" => {
                        memory.insert(var1.to_string(), *memory.get(var1).unwrap_or(&0) / val(&memory));
                    },
                    "mod" => {
                        memory.insert(var1.to_string(), *memory.get(var1).unwrap_or(&0) % val(&memory));
                    },
                    "eql" => {
                        memory.insert(var1.to_string(), (*memory.get(var1).unwrap_or(&0) == val(&memory)) as i64);
                    },
                    _ => ()
                }
                //println!("{}", &contents);
                //dbg!(&memory);

            }
        }
        dbg!(test_digit(input[0], z));
        dbg!(memory);
    }
}
