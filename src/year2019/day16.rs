use crate::utils::*;


pub struct Pattern {
    size: usize,
    prev: usize
}

impl Pattern {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            prev: 0
        }
    }
}

static BASE: [i64; 4] = [0, 1, 0, -1];

impl Iterator for Pattern {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {

        let next = self.prev + 1;

        let i = (next / self.size) % 4;

        self.prev = next;

        Some(BASE[i])
    }
}

pub fn fft(input: &[i64]) -> Vec<i64> {
    let mut next = input.to_owned();
    for (i, val) in next.iter_mut().enumerate() {
        let pat = Pattern::new(i + 1);
        let sum: i64 = input.iter().zip(pat).map(|(&x, p)| {
            // println!("{}: x={} p={}", i, x, p);
            x * p
         }).sum();
        *val = sum.abs() % 10;
    }
    next
}

#[test]
pub fn day16_part1() {
    if let Ok(lines) = read_lines("./src/year2019/data/day16input.txt") {
        let mut nums = None;
        
        // Consumes the iterator, returns an (Optional) String
        for (_line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                let n: Vec<i64> = contents.split("").filter(|s| s.len() > 0).map(|x| x.parse().unwrap()).collect();
                nums = Some(n);
            }
        }
        let mut nums = nums.unwrap();


        for _ in 0..100 {
            nums = fft(&nums);
        }

        dbg!(&nums);
    }
}


pub fn day16_part2() {
    if let Ok(lines) = read_lines("./src/year2019/data/day16testinput.txt") {
        let mut nums = None;
        
        // Consumes the iterator, returns an (Optional) String
        for (_line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                let n: Vec<i64> = contents.split("").filter(|s| s.len() > 0).map(|x| x.parse().unwrap()).collect();
                nums = Some(n);
            }
        }
        let mut single_nums = nums.unwrap();

        let mut nums = Vec::with_capacity(single_nums.len() * 10000);
        for _ in 0..10000 {
            for &x in &single_nums {
                nums.push(x);
            }
        }

        println!("starting fft");


        for i in 0..100 {
            println!("{}", i);
            nums = fft(&nums);
        }

        let i = from_digits(&nums[0..8]) as usize;
        let val = from_digits(&nums[i..i+8]);
        dbg!(i, val);
    }
}