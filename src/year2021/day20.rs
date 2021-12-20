use crate::utils::*;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use rayon::iter::*;

pub struct InfiniteImage {
    pub pxs: HashMap<IVec2, u8>,
    pub default: u8,
}

impl InfiniteImage {
    pub fn pretty_print(&self) {
        let max_y = self.max_y();
        let max_x = self.max_x();
        let min_x = self.min_x().min(0);
        let min_y = self.min_y().min(0);
        for y in min_y..=max_y {
            let mut row = vec![];
            for x in min_x..=max_x {
                let pos = IVec2::new(x, y);
                if *self.pxs.get(&pos).unwrap_or(&self.default) == 1 {
                    row.push("#")
                } else {
                    row.push(".")
                }
            }
            println!("{}", row.iter().join(""));
        }
    }

    pub fn max_x(&self) -> i32 {
        self.pxs.iter().map(|pos| pos.0.x).max().unwrap()
    }
    pub fn max_y(&self) -> i32 {
        self.pxs.iter().map(|pos| pos.0.y).max().unwrap()
    }

    pub fn min_x(&self) -> i32 {
        self.pxs.iter().map(|pos| pos.0.x).min().unwrap()
    }
    pub fn min_y(&self) -> i32 {
        self.pxs.iter().map(|pos| pos.0.y).min().unwrap()
    }

    pub fn enhance(&self, data: &[u8]) -> InfiniteImage {
        let blinks = data[0] != data[data.len() - 1] && data[0] > 0;
        let min_px = IVec2::new(i32::MAX, i32::MAX);
        let max_px = IVec2::new(i32::MIN, i32::MIN);
        let recolor = |px: IVec2| {
            let mut shift = 8;
            let mut num = 0usize;
            for pos in get_neighbors_and_pos(px) {
                let color = *self.pxs.get(&pos).unwrap_or(&self.default);
                num = num | ((color as usize) << shift);
                shift -= 1;
            }
            let color = data[num];
            color
        };
        let (min_px, max_px) = self
            .pxs
            .par_keys()
            .fold(
                || (min_px, max_px),
                |acc, &px| (acc.0.min(px), acc.1.max(px)),
            )
            .reduce(|| (min_px, max_px), |x, y| (x.0.min(y.0), x.1.max(y.1)));
        let mut img: HashMap<_, _> = self
            .pxs
            .par_keys()
            .map(|&source_px| (source_px, recolor(source_px)))
            .collect();

        let default = if blinks {
            for x in min_px.x - 1..=max_px.x + 1 {
                let px = IVec2::new(x, min_px.y - 1);
                img.insert(px, recolor(px));
                let px = IVec2::new(x, max_px.y + 1);
                img.insert(px, recolor(px));
            }
            for y in min_px.y - 1..=max_px.y + 1 {
                let px = IVec2::new(min_px.x - 1, y);
                img.insert(px, recolor(px));
                let px = IVec2::new(max_px.x + 1, y);
                img.insert(px, recolor(px));
            }

            (self.default + 1) % 2
        } else {
            self.default
        };

        InfiniteImage { pxs: img, default }
    }
}

pub fn day20() {
    if let Ok(lines) = read_lines("./src/year2021/data/day20input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut img: HashMap<IVec2, u8> = HashMap::new();
        let mut data = vec![];
        for (y, line) in lines.enumerate() {
            if let Ok(contents) = line {
                if y == 0 {
                    data = contents
                        .split("")
                        .filter(|s| s.len() > 0)
                        .map(|s| match s {
                            "#" => 1u8,
                            _ => 0u8,
                        })
                        .collect();
                }
                if y > 1 {
                    let y = y - 2;
                    for (x, color) in contents
                        .split("")
                        .filter(|s| s.len() > 0)
                        .map(|s| match s {
                            "#" => 1u8,
                            _ => 0u8,
                        })
                        .enumerate()
                    {
                        img.insert(IVec2::new(x as i32, y as i32), color);
                    }
                }
            }
        }

        let mut pic = InfiniteImage {
            pxs: img,
            default: 0,
        };

        for _ in 0..50 {
            pic = pic.enhance(&data);
        }
        dbg!(pic.pxs.values().filter(|&&x| x > 0).count());
    }
}
