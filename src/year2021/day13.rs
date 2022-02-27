use std::collections::{HashMap, HashSet};

use crate::utils::*;

pub fn fold_paper(paper: &HashSet<IVec2>, axis: usize, val: i32) -> HashSet<IVec2> {
    paper.iter().fold(HashSet::new(), |mut acc, elem| {
        let new_elem = if elem[axis] < val {
            *elem
        } else {
            let mut v = *elem;
            v[axis] = (2 * val) - v[axis];
            v
        };
        acc.insert(new_elem);
        acc
    })
}

pub fn pretty_print(paper: &HashSet<IVec2>) {
    let max_x = paper.iter().map(|x| x.x).max().unwrap();
    let max_y = paper.iter().map(|x| x.x).max().unwrap();
    for y in 0..max_y {
        let mut row = vec![];
        for x in 0..=max_x {
            let pos = IVec2::new(x, y);
            if paper.contains(&pos) {
                row.push("#");
            } else {
                row.push(" ")
            }
        }
        println!("{}", row.join(" "));
    }
}

#[test]
pub fn day_13() {
    if let Ok(lines) = read_lines("./src/year2021/data/day13input.txt") {
        // Consumes the iterator, returns an (Optional) String

        let mut paper = HashSet::new();
        let mut folds = vec![];
        for (_line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                let pair: Vec<_> = contents.split(",").collect();
                if pair.len() > 1 {
                    let ints: Vec<i32> = pair.iter().map(|s| s.parse().unwrap()).collect();
                    paper.insert(IVec2::new(ints[0], ints[1]));
                } else {
                    let fold_line: Vec<_> = contents.split_whitespace().collect();
                    if fold_line.len() == 3 {
                        let fold: Vec<_> = fold_line[2].split("=").collect();
                        let axis: usize = match fold[0] {
                            "x" => 0,
                            _ => 1,
                        };
                        let val: i32 = fold[1].parse().unwrap();
                        folds.push((axis, val));
                    }
                }
            }
        }
        let (axis, val) = folds[0];
        let first_fold = fold_paper(&paper, axis, val);
        dbg!(first_fold.len());
        for (axis, val) in folds {
            paper = fold_paper(&paper, axis, val)
        }
        pretty_print(&paper);
    }
}
