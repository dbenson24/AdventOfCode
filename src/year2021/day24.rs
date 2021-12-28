use crate::utils::*;
use core::cmp::Ordering;
use hashbrown::{HashMap, HashSet};
use std::collections::BinaryHeap;
use std::num::Wrapping;
use rayon::iter::*;


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

#[test]
pub fn test_fns() {
    for dig in 1..=9 {
        for z in 0..1000 {
            assert_eq!(test_digit_first_input(dig, z), run_original(dig, z));
        }
    }
}

pub fn find_highest_passing() {
    //let z = test_ith_digit(1, 0, 0);
    let x = test_z(0, 0);
    dbg!(x);
}

pub fn test_z(i: usize, z: i64) -> Option<i64> {
    if i < 3 {
        println!("{}{}", format!("{:width$}", "", width=i), i);
    }
    const DIGITS: [i64; 9] = [1,2,3,4,5,6,7,8,9];
    if i == 4 || i == 5 {
        let x: Vec<_> = DIGITS.into_par_iter().fold(|| None, |mut acc: Option<i64>, digit| {
            if acc.is_some() {
                return acc;
            }
    
            let next_z = test_ith_digit(digit, z, i);
            if i == 13 {
                if next_z == 0 {
                    return Some(digit);
                }
            } else {
                if let Some(num) = test_z(i + 1, next_z) {
                    let val = 10i64.pow(13 - i as u32) * digit;
                    return Some(num + val)
                }
            }
            None
        }).collect();
        for y in x {
            if y.is_some() {
                return y
            }
        }
        None
    } else {
        (1..=9).fold(None, |mut acc, digit| {
            if acc.is_some() {
                return acc;
            }
    
            let next_z = test_ith_digit(digit, z, i);
            if i == 13 {
                if next_z == 0 {
                    return Some(digit);
                }
            } else {
                if let Some(num) = test_z(i + 1, next_z) {
                    let val = 10i64.pow(13 - i as u32) * digit;
                    return Some(num + val)
                }
            }
            None
        })
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
        dbg!(memory);
        let mut test_z = 0;
        for (i, &digit) in input.iter().enumerate() {
            test_z = test_ith_digit(digit, test_z, i)
        }
        dbg!(test_z);
    }
}

pub fn test_digit_first_input(digit: i64, z: i64) -> i64 {
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