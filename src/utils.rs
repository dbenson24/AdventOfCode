pub use glam::{IVec2, IVec3, Quat, Vec2, Vec3};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use pad::PadStr;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Clone)]
pub struct VecWorld<T: Default> {
    data: Vec<Vec<T>>,
}

impl<T: Default + Clone> VecWorld<T> {
    pub fn new() -> Self {
        VecWorld {
            data: vec![vec![T::default(); 50000]; 50000],
        }
    }

    pub fn get(&self, pos: IVec2) -> &T {
        &self.data[(pos.x + 25000) as usize][(pos.y + 25000) as usize]
    }

    pub fn set(&mut self, pos: IVec2, val: T) {
        self.data[(pos.x + 25000) as usize][(pos.y + 25000) as usize] = val;
    }
}

pub fn get_neighbors(pos: IVec2) -> [IVec2; 8] {
    let x = pos.x;
    let y = pos.y;
    let mut i = 0;
    let mut neighs: [IVec2; 8] = Default::default();
    for y_i in y - 1..y + 2 {
        for x_i in x - 1..x + 2 {
            if !(x == x_i && y == y_i) {
                neighs[i] = IVec2::new(x_i, y_i);
                i += 1;
            }
        }
    }
    neighs
}

pub fn get_neighbors_and_pos(pos: IVec2) -> [IVec2; 9] {
    let x = pos.x;
    let y = pos.y;
    let mut i = 0;
    let mut neighs: [IVec2; 9] = Default::default();
    for y_i in y - 1..y + 2 {
        for x_i in x - 1..x + 2 {
            neighs[i] = IVec2::new(x_i, y_i);
            i += 1;
        }
    }
    neighs
}

pub fn get_cardinal_neighbors(pos: IVec2) -> [IVec2; 4] {
    let x = pos.x;
    let y = pos.y;

    [
        IVec2::new(x - 1, y),
        IVec2::new(x + 1, y),
        IVec2::new(x, y - 1),
        IVec2::new(x, y + 1),
    ]
}
pub struct World<T> {
    pub world: HashMap<IVec2, T>,
}

impl<T: std::fmt::Debug + std::str::FromStr + Default + std::fmt::Display> World<T> {
    pub fn from_file(path: &str) -> Option<Self> {
        if let Ok(lines) = read_lines(path) {
            let mut world = HashMap::new();
            for (y, line) in lines.enumerate() {
                if let Ok(contents) = line {
                    for (x, height) in contents
                        .split("")
                        .filter(|s| s.len() > 0)
                        .map(|s| s.parse::<T>().unwrap_or_default())
                        .enumerate()
                    {
                        let pos = IVec2::new(x as i32, y as i32);
                        world.insert(pos, height);
                    }
                }
            }
            Some(World { world })
        } else {
            None
        }
    }

    pub fn pretty_print(&self) {
        let max_y = self.max_y();
        let max_x = self.max_x();
        let min_x = self.min_x().min(0);
        let min_y = self.min_y().min(0);
        for y in min_y..=max_y {
            let mut row = vec![];
            for x in min_x..=max_x {
                let pos = IVec2::new(x, y);
                if let Some(val) = self.world.get(&pos) {
                    row.push(val.to_string().pad_to_width(5));
                } else {
                    row.push(" ".to_string().pad_to_width(5));
                }
            }
            println!("{}", row.iter().join(""));
        }
    }

    pub fn max_x(&self) -> i32 {
        self.world.keys().map(|pos| pos.x).max().unwrap()
    }
    pub fn max_y(&self) -> i32 {
        self.world.keys().map(|pos| pos.y).max().unwrap()
    }

    pub fn min_x(&self) -> i32 {
        self.world.keys().map(|pos| pos.x).min().unwrap()
    }
    pub fn min_y(&self) -> i32 {
        self.world.keys().map(|pos| pos.y).min().unwrap()
    }
}

pub fn pretty_print_set(set: &HashSet<IVec2>, str_fn: &Fn(&IVec2) -> String, width: usize) {
    let max_y = set.iter().map(|pos| pos.y).max().unwrap();
    let max_x = set.iter().map(|pos| pos.x).max().unwrap();
    for y in 0..=max_y {
        let mut row = vec![];
        for x in 0..=max_x {
            let pos = IVec2::new(x, y);
            if let Some(val) = set.get(&pos) {
                row.push(str_fn(val).pad_to_width(width));
            } else {
                row.push(" ".to_string().pad_to_width(width));
            }
        }
        println!("{}", row.iter().join(""));
    }
}

#[test]
pub fn base() {
    if let Ok(lines) = read_lines("./src/year2021/data/day1testinput.txt") {
        // Consumes the iterator, returns an (Optional) String
        for (line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {}
        }
    }
}
