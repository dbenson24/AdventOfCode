use crate::utils::*;

pub fn eval_int_code(numbers: &mut Vec<usize>) -> usize {
    let mut pos = 0;
    loop {
        let op = numbers[pos];
        match op {
            1 => {
                let a = numbers[pos + 1];
                let b = numbers[pos + 2];
                let dest = numbers[pos + 3];
                //dbg!(a,b,a+b, dest);
                numbers[dest] = numbers[a] + numbers[b];
                pos += 4;
            }
            2 => {
                let a = numbers[pos + 1];
                let b = numbers[pos + 2];
                let dest = numbers[pos + 3];
                //dbg!(a,b,a*b, dest);
                numbers[dest] = numbers[a] * numbers[b];
                pos += 4;
            }
            99 => {
                return numbers[0];
            }
            _ => {
                panic!("hit bad opcode {} at {}", op, pos);
            }
        }
    }
}

#[test]
pub fn run_opcodes() {
    if let Ok(lines) = read_lines("./src/year2019/data/day2input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for (_line_num, line) in lines.enumerate() {
            let mut numbers: Vec<usize> = line
                .unwrap()
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect();
            for a in 0..100 {
                for b in 0..100 {
                    let mut modified_numbers = numbers.clone();
                    modified_numbers[1] = a;
                    modified_numbers[2] = b;
                    let res = eval_int_code(&mut modified_numbers);
                    if res == 19690720 {
                        dbg!(a, b, res);
                    }
                }
            }
        }
    }
}
