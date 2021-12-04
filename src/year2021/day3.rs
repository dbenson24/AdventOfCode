use crate::utils::*;
use std::str::FromStr;
use std::convert::From;


#[test]
fn calculate_gamma_epsilon() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./src/year2021/data/day3input.txt") {
        let mut totals: Vec<i32> = Vec::new();
        let mut line_count = 0;

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(contents) = line {
                if totals.len() == 0 {
                    totals.resize(contents.len(), 0);
                    dbg!(contents.len());
                }
                let int = i32::from_str_radix(&contents, 2).unwrap();

                for (i, x) in totals.iter_mut().enumerate() {
                    let bit = int & (1 << i);
                    *x += bit >> i;
                }

                line_count += 1;
            }

        }
        
        let mut gamma = 0;
        let mut epsilon = 0;
        let half = line_count / 2;
        for (i, &total) in totals.iter().enumerate() {
            if total > half {
                gamma += 1 << i;
            } else {
                epsilon += 1 << i;
            }
        }
        dbg!(gamma, epsilon, gamma * epsilon);
    }
}

fn calculate_most_common_bit(nums: &[i32], bit_count: usize) -> Vec<i32> {
    let mut totals = vec![0; bit_count];

    for &int in nums {
        for (i, x) in totals.iter_mut().enumerate() {
            let bit = int.get_bit(i, 32 - bit_count);
            *x += bit;
        }
    }

    for x in totals.iter_mut() {
        *x = if *x as f32 >= nums.len() as f32 / 2. {
            1
        } else {
            0
        }
    }

    totals
}

fn to_ints(strs: &[&str]) -> Vec<i32> {
    strs.iter().map(|text| text.parse().unwrap()).collect()
}

trait get_bit {
    fn get_bit(&self, i: usize, first_bit: usize) -> i32;
}

impl get_bit for i32 {
    fn get_bit(&self, i: usize, first_bit: usize) -> i32 {
        let i = 31 - (first_bit + i);
        (*self & (1 << i)) >> i
    }
}




#[test]
fn calculate_oxy_and_co2() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./src/year2021/data/day3input.txt") {
        let mut bit_count = 0;
        let mut totals: Vec<i32> = Vec::new();
        let mut line_count = 0;
        let mut numbers = vec![];

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(contents) = line {
                if totals.len() == 0 {
                    bit_count = contents.len();
                    totals.resize(contents.len(), 0);
                    dbg!(contents.len());
                }
                let int = i32::from_str_radix(&contents, 2).unwrap();
                numbers.push(int);

                line_count += 1;
            }
        }

        totals = calculate_most_common_bit(&numbers, bit_count);
        dbg!(&totals);

        // calc oxy rating
        let mut oxy_values = numbers.clone();
        for i in 0..totals.len() {
            let filtered_totals = calculate_most_common_bit(&oxy_values, bit_count);
            oxy_values = oxy_values.into_iter().filter(|&val| {
                let bit = val.get_bit(i, 32 - bit_count);
                bit == filtered_totals[i]
            }).collect();
            
            if oxy_values.len() == 1 {
                break;
            }
        }

        
        // calc oxy rating
        let mut co2_values = numbers.clone();
        for i in 0..totals.len() {
            let mut filtered_totals = calculate_most_common_bit(&co2_values, bit_count);
            filtered_totals.iter_mut().for_each(|x| *x = (*x - 1).abs());
            dbg!(&co2_values, &filtered_totals);
            co2_values = co2_values.into_iter().filter(|&val| {
                let bit = val.get_bit(i, 32 - bit_count);
                bit == filtered_totals[i]
            }).collect();
            if co2_values.len() == 1 {
                break;
            }
        }

        let mut gamma = 0;
        let mut epsilon = 0;
        for (i, &bit) in totals.iter().enumerate() {
            if bit > 0 {
                gamma += 1 << (bit_count - i - 1);
            } else {
                epsilon += 1 << (bit_count - i - 1);
            }
        }
        dbg!(gamma, epsilon, gamma * epsilon);
        
        dbg!(&oxy_values);
        dbg!(&co2_values);
        dbg!(oxy_values[0] * co2_values[0]);
    }
}
