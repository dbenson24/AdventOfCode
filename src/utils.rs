use glam::IVec2;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use pad::PadStr;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub type Vec2 = IVec2;

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

    pub fn get(&self, pos: Vec2) -> &T {
        &self.data[(pos.x + 25000) as usize][(pos.y + 25000) as usize]
    }

    pub fn set(&mut self, pos: Vec2, val: T) {
        self.data[(pos.x + 25000) as usize][(pos.y + 25000) as usize] = val;
    }
}

pub fn get_neighbors(pos: Vec2) -> [Vec2; 8] {
    let x = pos.x;
    let y = pos.y;
    let mut i = 0;
    let mut neighs: [Vec2; 8] = Default::default();
    for x_i in x - 1..x + 2 {
        for y_i in y - 1..y + 2 {
            if !(x == x_i && y == y_i) {
                neighs[i] = Vec2::new(x_i, y_i);
                i += 1;
            }
        }
    }
    neighs
}

pub fn get_cardinal_neighbors(pos: Vec2) -> [Vec2; 4] {
    let x = pos.x;
    let y = pos.y;

    [
        Vec2::new(x - 1, y),
        Vec2::new(x + 1, y),
        Vec2::new(x, y - 1),
        Vec2::new(x, y + 1),
    ]
}
pub struct World<T> {
    pub world: HashMap<Vec2, T>,
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
                        let pos = Vec2::new(x as i32, y as i32);
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
        for y in 0..=max_y {
            let mut row = vec![];
            for x in 0..=max_x {
                let pos = Vec2::new(x, y);
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
