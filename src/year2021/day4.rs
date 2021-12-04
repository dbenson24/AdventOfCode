use crate::utils::*;
use std::str::FromStr;
use std::convert::From;

#[derive(Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Board {
    pub nums: [[(i32, bool); 5]; 5]
}

impl Board {
    pub fn mark_num(&mut self, x: i32) {
        for row in self.nums.iter_mut() {
            for col in row.iter_mut() {
                if col.0 == x {
                    col.1 = true
                }
            }
        }
    }
    pub fn has_won(&self) -> bool {
        for row in 0..5 {
            let mut trues = 0;
            for col in 0..5 {
                if self.nums[row][col].1 {
                    trues += 1;
                }
            }
            if trues == 5 {
                return true;
            }
        }
        
        for col in 0..5 {
            let mut trues = 0;
            for row in 0..5 {
                if self.nums[row][col].1 {
                    trues += 1;
                }
            }
            if trues == 5 {
                return true;
            }
        }

        false
    }

    pub fn unmarked_sum(&self) -> i32 {
        self.nums.iter().map(|row| row.iter().map(|(x, called)| if *called { 0 } else { *x }).sum::<i32>()).sum()
    }
}




#[test]
fn calc_bingo_winners() {
    if let Ok(lines) = read_lines("./src/year2021/data/day4input.txt") {

        let mut nums_to_call: Vec<i32> = vec![];
        let mut row_num: i32 = -1;
        let mut boards = vec![];
        let mut curr_board = Board::default();
        // Consumes the iterator, returns an (Optional) String
        for (line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                if line_num == 0 {
                    nums_to_call = contents.split(",").map(|s| {
                        let x = s.parse();
                        dbg!(&x, s);
                        x.unwrap()
                    }).collect();
                } else {
                    if row_num >= 0 {
                        dbg!(&contents);
                        let cols: Vec<(i32, bool)> = contents.split(" ").filter(|s| s.len() > 0).map(|s| (s.parse().unwrap(), false)).collect();
                        for (i, num) in curr_board.nums[row_num as usize].iter_mut().enumerate() {
                            *num = cols[i];
                        }
                    }
                    row_num += 1;
                    if row_num == 5  {
                        boards.push(curr_board);
                        curr_board = Board::default();
                        row_num = -1;
                    }
                }
            }
        }

        for (i, &x) in nums_to_call.iter().enumerate() {
            for board in &mut boards {
                board.mark_num(x);
            }

            if i >= 4 {
                for board in &boards {
                    if board.has_won() {
                        let sum = board.unmarked_sum();
                        dbg!(sum, x, sum * x);
                        return
                    }
                }
            }
        }
        
        
    }
}


#[test]
fn calc_bingo_losers() {
    if let Ok(lines) = read_lines("./src/year2021/data/day4input.txt") {

        let mut nums_to_call: Vec<i32> = vec![];
        let mut row_num: i32 = -1;
        let mut boards = vec![];
        let mut curr_board = Board::default();
        // Consumes the iterator, returns an (Optional) String
        for (line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                if line_num == 0 {
                    nums_to_call = contents.split(",").map(|s| {
                        let x = s.parse();
                        dbg!(&x, s);
                        x.unwrap()
                    }).collect();
                } else {
                    if row_num >= 0 {
                        let cols: Vec<(i32, bool)> = contents.split(" ").filter(|s| s.len() > 0).map(|s| (s.parse().unwrap(), false)).collect();
                        for (i, num) in curr_board.nums[row_num as usize].iter_mut().enumerate() {
                            *num = cols[i];
                        }
                    }
                    row_num += 1;
                    if row_num == 5  {
                        boards.push(curr_board);
                        curr_board = Board::default();
                        row_num = -1;
                    }
                }
            }
        }

        for (i, &x) in nums_to_call.iter().enumerate() {
            for board in &mut boards {
                board.mark_num(x);
            }

            if i >= 4 {

                if boards.len() == 1 && boards[0].has_won() {
                    let sum = boards[0].unmarked_sum();
                    dbg!(sum, x, sum * x);
                    return
                }

                boards = boards.into_iter().filter(|b| !b.has_won()).collect();
            }
        }
    }
}





