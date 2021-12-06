
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use glam::IVec2;

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
pub struct World<T: Default> {
    data: Vec<Vec<T>>
}

impl<T: Default + Clone> World<T> {
    pub fn new() -> Self {
        World {
            data: vec![vec![T::default(); 50000]; 50000]
        }
    }

    pub fn get(&self, pos: Vec2) -> &T {
        &self.data[(pos.x + 25000) as usize][(pos.y + 25000) as usize]
    }

    pub fn set(&mut self, pos: Vec2, val: T) {
        self.data[(pos.x + 25000) as usize][(pos.y + 25000) as usize] = val;
    }
}


#[test]
pub fn base() {
    if let Ok(lines) = read_lines("./src/year2021/data/day1testinput.txt") {
        // Consumes the iterator, returns an (Optional) String
        for (line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {

            }
        } 
    }
}

