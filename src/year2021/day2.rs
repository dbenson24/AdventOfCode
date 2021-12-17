use crate::utils::*;
use std::convert::From;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    UP,
    DOWN,
    FORWARD,
    BACKWARD,
}

impl From<Direction> for IVec2 {
    fn from(dir: Direction) -> IVec2 {
        match dir {
            Direction::UP => IVec2::new(0, -1),
            Direction::DOWN => IVec2::new(0, 1),
            Direction::FORWARD => IVec2::new(1, 0),
            Direction::BACKWARD => IVec2::new(-1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Movement {
    dir: Direction,
    dist: i32,
}
impl FromStr for Movement {
    type Err = ParseMovementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut count = 0;
        let mut dir: Direction = Direction::FORWARD;
        let mut dist: i32 = 0;
        for word in s.split(" ") {
            if count == 0 {
                dir = word.parse().unwrap();
            } else if count == 1 {
                dist = word.parse().unwrap();
            } else {
                return Err(ParseMovementError);
            }
            count += 1;
        }
        Ok(Movement { dir, dist })
    }
}

impl Movement {
    pub fn get_move(&self) -> IVec2 {
        let dir: IVec2 = self.dir.into();
        dir * self.dist
    }
}

#[derive(Debug, Clone)]
struct ParseDirectionError;

#[derive(Debug, Clone)]
struct ParseMovementError;

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Direction::FORWARD),
            "backward" => Ok(Direction::BACKWARD),
            "up" => Ok(Direction::UP),
            "down" => Ok(Direction::DOWN),
            _ => Err(ParseDirectionError),
        }
    }
}

#[test]
fn track_movement() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./src/year2021/data/day2input.txt") {
        let mut pos = IVec2::new(0, 0);
        let mut aim = 0;
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(contents) = line {
                let movement: Movement = contents.parse().unwrap();

                if movement.dir == Direction::FORWARD {
                    pos.x += movement.dist;
                    pos.y += aim * movement.dist;
                } else {
                    let dir: IVec2 = movement.dir.into();
                    aim += dir.y * movement.dist;
                }
            }
        }
        dbg!(pos.x * pos.y);
    }
}
