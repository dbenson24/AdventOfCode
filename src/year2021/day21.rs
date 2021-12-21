use crate::utils::*;
use hashbrown::{HashMap, HashSet};
use image::ImageBuffer;
use itertools::Itertools;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use rayon::iter::*;

pub fn game_wins(
    p1: usize,
    p1_score: usize,
    p2: usize,
    p2_score: usize,
    board: &[usize],
    memo: &mut HashMap<(usize, usize, usize, usize), (usize, usize)>,
) -> (usize, usize) {
    let mut wins = (0, 0);
    for p1_1 in 1..4 {
        for p1_2 in 1..4 {
            for p1_3 in 1..4 {
                let p1_move = p1_1 + p1_2 + p1_3;
                let new_p1 = (p1 + p1_move) % board.len();
                let new_p1_score = p1_score + board[new_p1];

                if new_p1_score >= 21 {
                    wins.0 += 1;
                } else {
                    for p2_1 in 1..4 {
                        for p2_2 in 1..4 {
                            for p2_3 in 1..4 {
                                let p2_move = p2_1 + p2_2 + p2_3;
                                let new_p2 = (p2 + p2_move) % board.len();
                                let new_p2_score = p2_score + board[new_p2];

                                if new_p2_score >= 21 {
                                    wins.1 += 1;
                                } else {
                                    let key = (new_p1, new_p1_score, new_p2, new_p2_score);
                                    if let Some(res) = memo.get(&key) {
                                        wins.0 += res.0;
                                        wins.1 += res.1;
                                    } else {
                                        let res = game_wins(
                                            new_p1,
                                            new_p1_score,
                                            new_p2,
                                            new_p2_score,
                                            board,
                                            memo,
                                        );
                                        memo.insert(key, res);
                                        wins.0 += res.0;
                                        wins.1 += res.1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    wins
}

pub fn day21() {
    let board = (1..=10).collect_vec();
    let mut p1 = 2;
    let mut p1_score = 0;
    let mut p2 = 6;
    let mut p2_score = 0;

    let dice = (1..=100).collect_vec();
    let mut dice_rolls = 0;
    let mut curr_roll = 0;

    loop {
        let p1_move = dice[curr_roll]
            + dice[(curr_roll + 1) % dice.len()]
            + dice[(curr_roll + 2) % dice.len()];
        curr_roll = (curr_roll + 3) % dice.len();
        dice_rolls += 3;

        p1 = (p1 + p1_move) % board.len();
        p1_score += board[p1];

        if p1_score >= 1000 {
            dbg!(p2_score * dice_rolls);
            break;
        }

        let p2_move = dice[curr_roll]
            + dice[(curr_roll + 1) % dice.len()]
            + dice[(curr_roll + 2) % dice.len()];
        curr_roll = (curr_roll + 3) % dice.len();
        dice_rolls += 3;

        p2 = (p2 + p2_move) % board.len();

        p2_score += board[p2];

        if p2_score >= 1000 {
            dbg!(p1_score * dice_rolls);
            break;
        }
    }

    dbg!(game_wins(2, 0, 6, 0, &board, &mut HashMap::new()));
}
