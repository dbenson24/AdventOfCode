use rayon::prelude::*;

use crate::utils::*;

pub struct Pattern {
    size: usize,
    prev: usize,
}

impl Pattern {
    pub fn new(size: usize) -> Self {
        Self { size, prev: 0 }
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

#[derive(Debug, Clone)]
pub struct RangeSumTree {
    levels: Vec<Vec<RangeSum>>,
}

impl RangeSumTree {
    pub fn new(vals: &[i64]) -> Self {
        let depth = (vals.len() as f64).log2().ceil() as usize;
        let mut levels = vec![vec![]; depth];

        for (idx, &sum) in vals.iter().enumerate() {
            levels[0].push(RangeSum { sum })
        }

        for i in 1..depth {
            let range_len = 2 * i;
            let half_len = range_len / 2;
            let mut idx = 0;
            while idx < vals.len() {
                let mut sum = RangeSum { sum: 0 };
                let left_child = idx / half_len;
                let right_child = left_child + 1;
                if left_child < levels[i - 1].len() {
                    sum.sum += levels[i - 1][left_child].sum
                }

                if right_child < levels[i - 1].len() {
                    sum.sum += levels[i - 1][right_child].sum
                }

                levels[i].push(sum);

                idx += range_len;
            }
        }
        Self { levels }
    }

    pub fn sum_range(&self, start: usize, len: usize) -> i64 {
        self.sum_range_depth(start, len, self.levels.len() - 1)
    }
    fn sum_range_depth(&self, start: usize, len: usize, depth: usize) -> i64 {
        let span_size = 2usize.pow(depth as u32);
        let mut sum = 0;
        let mut i = start;
        let end = start + len;
        while i < start + len {
            let chunk_idx = i / span_size;
            let chunk_start = chunk_idx * span_size;
            let chunk_end = chunk_start + span_size;
            let chunk_len = (end - i).min(span_size);

            if chunk_start < i {
                sum += self.sum_range_depth(i, chunk_len, depth - 1);
            } else if chunk_start == i {
                if chunk_len == span_size {
                    sum += self.levels[depth][chunk_idx].sum
                } else {
                    sum += self.sum_range_depth(i, chunk_len, depth - 1);
                }
            } else {
                println!("shouldn't get hit");
            }

            i += chunk_len;
        }

        sum
    }
}

#[test]
fn test_sums() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];

    let tree = RangeSumTree::new(&nums);

    dbg!(tree.sum_range(3, 4));
    let sum: i64 = nums[3..7].iter().sum();
    dbg!(sum);
}

#[derive(Debug, Clone)]
pub struct RangeSum {
    pub sum: i64,
}

pub fn fft(input: &[i64]) -> Vec<i64> {
    let mut next = input.to_owned();
    for (i, val) in next.iter_mut().enumerate() {
        println!("{:?} / {:?}", i, input.len());

        let pattern_size = i + 1;
        let mut idx = 0;
        let mut sum = 0;
        while idx < input.len() {
            let pat_idx = idx + 1;
            let pat_num = (pat_idx / pattern_size);
            let pat_val = BASE[pat_num % 4];
            if pat_val == 0 {
                let skip = pattern_size - (pat_idx % pattern_size);
                idx += skip;
            } else {
                sum += pat_val * input[idx];
                idx += 1;
            }
        }
        *val = sum.abs() % 10;
    }
    next
}

pub fn fft_span(input: &[i64], offset: usize) -> Vec<i64> {
    let mut next = input.to_owned();
    let tree = RangeSumTree::new(input);
    let pct = (input.len() / 8).max(1);

    next.par_iter_mut().enumerate().rev().for_each(|(i, val)| {
        if i % pct == 0 {
            // println!("{:?} / {:?}", i, input.len());
        }

        let pattern_size = i + 1 + offset;
        let mut idx = 0;
        let mut sum = 0;
        while idx < input.len() {
            let pat_idx = idx + 1 + offset;
            let pat_num = pat_idx / pattern_size;
            let pat_val = BASE[pat_num % 4];
            let curr_size = (pattern_size - (pat_idx % pattern_size)).min(input.len() - idx);
            if pat_val != 0 {
                sum += pat_val * tree.sum_range(idx, curr_size);
            }
            idx += curr_size;
        }
        *val = sum.abs() % 10;
    });
    next
}

// pub fn fft_chunks(input: &[i64]) -> Vec<i64> {
//     let mut next = input.to_owned();

//     let mut chunks = Vec::new();
//     let mut first_sum = 0;
//     for (i , (&val, pat_val)) in input.iter().zip(Pattern::new(1)).enumerate() {
//         chunks.push((i, val, pat_val));
//         first_sum += val * pat_val;
//     }
//     next[0] = first_sum.abs() % 10;

//     let idx = 1;
//     while idx < input.len() {
//         let curr_chunks = Vec::new();
//         let mut shift = 1;
//         for (i, sum, pat_val) in chunks {

//         }
//     }

//     for (i, val) in next.iter_mut().enumerate() {
//         println!("{:?} / {:?}", i, input.len());

//         let pattern_size = i + 1;
//         let mut idx = 0;
//         let mut sum = 0;
//         while idx < input.len() {
//             let pat_idx = idx + 1;
//             let pat_num = (pat_idx / pattern_size);
//             let pat_val = BASE[pat_num % 4];
//             if pat_val == 0 {
//                 let skip = pattern_size - (pat_idx % pattern_size);
//                 idx += skip;
//             } else {
//                 sum += pat_val * input[idx];
//                 idx += 1;
//             }
//         }
//         *val = sum.abs() % 10;
//     }
//     next
// }

pub fn repeat_slice<T>(nums: &[T], count: usize) -> Vec<T>
where
    T: Clone,
{
    let mut ret = Vec::new();
    for _ in 0..count {
        for x in nums {
            ret.push(x.clone());
        }
    }
    ret
}

#[test]
pub fn day16_part1() {
    if let Ok(lines) = read_lines("./src/year2019/data/day16input.txt") {
        let mut nums = None;

        // Consumes the iterator, returns an (Optional) String
        for (_line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                let n: Vec<i64> = contents
                    .split("")
                    .filter(|s| s.len() > 0)
                    .map(|x| x.parse().unwrap())
                    .collect();
                nums = Some(n);
            }
        }
        let mut nums = nums.unwrap();

        for _ in 0..100 {
            nums = fft_span(&nums, 0);
        }

        println!("{:?}", &nums);
    }
}

pub fn day16_part2() {
    if let Ok(lines) = read_lines("./src/year2019/data/day16input.txt") {
        let mut nums = None;

        // Consumes the iterator, returns an (Optional) String
        for (_line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                let n: Vec<i64> = contents
                    .split("")
                    .filter(|s| s.len() > 0)
                    .map(|x| x.parse().unwrap())
                    .collect();
                nums = Some(n);
            }
        }
        let mut single_nums = nums.unwrap();

        let res_idx = from_digits(&single_nums[0..7]) as usize;
        dbg!(res_idx);

        let nums = repeat_slice(&single_nums, 10000);
        dbg!(nums.len(), single_nums.len());
        let mut nums = nums[res_idx..single_nums.len() * 10000].to_owned();
        dbg!(nums.len());

        println!("starting fft");

        for i in 0..100 {
            println!("{}", i);
            //nums = fft_span(&nums, res_idx);
            nums.iter_mut().rev().fold(0, |mut acc, x| {
                acc += *x;
                *x = acc % 10;
                acc
            });
            // println!("{:?}", &nums);
        }
        let i = from_digits(&nums[0..8]) as usize;
        dbg!(i);
    }
}
