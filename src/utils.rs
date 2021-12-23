pub use glam::{IVec2, IVec3, Quat, Vec2, Vec3};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::*;
use pad::PadStr;
pub use smallvec::{smallvec, SmallVec};
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

impl<T> World<T> {
    pub fn from_file(path: &str, parse_fn: &dyn Fn(&str) -> T) -> Option<Self> {
        if let Ok(lines) = read_lines(path) {
            let mut world = HashMap::new();
            for (y, line) in lines.enumerate() {
                if let Ok(contents) = line {
                    for (x, height) in contents
                        .split("")
                        .filter(|s| s.len() > 0)
                        .map(parse_fn)
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

    pub fn pretty_print(&self, str_fn: &impl Fn(&T) -> String) {
        let max_y = self.max_y();
        let max_x = self.max_x();
        let min_x = self.min_x().min(0);
        let min_y = self.min_y().min(0);
        for y in (min_y..=max_y).rev() {
            let mut row = vec![];
            for x in min_x..=max_x {
                let pos = IVec2::new(x, y);
                if let Some(val) = self.world.get(&pos) {
                    row.push(str_fn(val));
                } else {
                    row.push(" ".to_string());
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
    let min_x = set.iter().map(|pos| pos.x).min().unwrap();
    let min_y = set.iter().map(|pos| pos.y).min().unwrap();
    for y in min_y..=max_y {
        let mut row = vec![];
        for x in min_x..=max_x {
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

#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dir3 {
    Left,
    Right,
    Up,
    Down,
    Forward,
    Backward,
}

impl Dir3 {
    pub fn flipped(&self) -> Dir3 {
        match self {
            Dir3::Forward => Dir3::Backward,
            Dir3::Backward => Dir3::Forward,
            Dir3::Left => Dir3::Right,
            Dir3::Right => Dir3::Left,
            Dir3::Up => Dir3::Down,
            Dir3::Down => Dir3::Up,
        }
    }
}

impl Into<Vec3> for &Dir3 {
    fn into(self) -> Vec3 {
        match self {
            Dir3::Left => Vec3::X * -1.,
            Dir3::Right => Vec3::X,
            Dir3::Up => Vec3::Y,
            Dir3::Down => Vec3::Y * -1.,
            Dir3::Forward => Vec3::Z,
            Dir3::Backward => Vec3::Z * -1.,
        }
    }
}
impl Into<Vec3> for Dir3 {
    fn into(self) -> Vec3 {
        (&self).into()
    }
}

#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dir2 {
    Left,
    Up,
    Right,
    Down,
}

impl Dir2 {
    pub fn flipped(&self) -> Dir2 {
        match self {
            Dir2::Left => Dir2::Right,
            Dir2::Right => Dir2::Left,
            Dir2::Up => Dir2::Down,
            Dir2::Down => Dir2::Up,
        }
    }

    pub fn turn_right(&self) -> Dir2 {
        let i = (ToPrimitive::to_i32(self).unwrap() + 1) % 4;
        FromPrimitive::from_i32(i).unwrap()
    }
    pub fn turn_left(&self) -> Dir2 {
        let i = (ToPrimitive::to_i32(self).unwrap() + 3) % 4;
        FromPrimitive::from_i32(i).unwrap()
    }
}

impl Into<Vec2> for &Dir2 {
    fn into(self) -> Vec2 {
        match self {
            Dir2::Left => Vec2::X * -1.,
            Dir2::Right => Vec2::X,
            Dir2::Up => Vec2::Y,
            Dir2::Down => Vec2::Y * -1.,
        }
    }
}
impl Into<Vec2> for Dir2 {
    fn into(self) -> Vec2 {
        (&self).into()
    }
}

impl Into<IVec2> for &Dir2 {
    fn into(self) -> IVec2 {
        match self {
            Dir2::Left => IVec2::X * -1,
            Dir2::Right => IVec2::X,
            Dir2::Up => IVec2::Y,
            Dir2::Down => IVec2::Y * -1,
        }
    }
}
impl Into<IVec2> for Dir2 {
    fn into(self) -> IVec2 {
        (&self).into()
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
