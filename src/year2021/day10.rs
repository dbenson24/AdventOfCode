use crate::utils::*;

pub fn is_valid_line(line: &str) -> (bool, Option<u8>, Option<usize>) {
    let mut stack = vec![];
    let opening = [b'(', b'[', b'{', b'<'];
    let closing = [b')', b']', b'}', b'>'];
    for byte in line.as_bytes() {
        if opening.contains(byte) {
            stack.push(byte);
        } else if let Some(i) = closing.iter().position(|x| x == byte) {
            let expected_byte = opening[i];
            if let Some(&opener) = stack.pop() {
                if expected_byte != opener {
                    return (false, Some(*byte), None);
                }
            } else {
                return (false, Some(*byte), None);
            }
        } else {
            return (false, Some(*byte), None);
        }
    }

    if stack.len() == 0 {
        (true, None, None)
    } else {
        let mut score = 0;
        while let Some(&byte) = stack.pop() {
            let i = opening.iter().position(|&x| x == byte).unwrap();
            let closer = closing[i];

            score *= 5;
            score += match closer {
                b')' => 1,
                b']' => 2,
                b'}' => 3,
                b'>' => 4,
                _ => 0,
            }
        }

        (true, None, Some(score))
    }
}

#[test]
pub fn day_10_part_1() {
    if let Ok(lines) = read_lines("./src/year2021/data/day10input.txt") {
        let mut score = 0;
        let mut completion_scores = vec![];
        // Consumes the iterator, returns an (Optional) String
        for (line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                let (valid, failed_byte, completion_score) = is_valid_line(&contents);
                if !valid {
                    if let Some(byte) = failed_byte {
                        score += match byte {
                            b')' => 3,
                            b']' => 57,
                            b'}' => 1197,
                            b'>' => 25137,
                            _ => 0,
                        }
                    }
                } else {
                    if let Some(val) = completion_score {
                        completion_scores.push(val);
                    }
                }
            }
        }
        completion_scores.sort();
        dbg!(score);
        dbg!(completion_scores[completion_scores.len() / 2]);
    }
}
