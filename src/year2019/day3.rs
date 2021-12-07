use crate::utils::*;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl From<Direction> for Vec2 {
    fn from(dir: Direction) -> Vec2 {
        match dir {
            Direction::UP => Vec2::new(0, 1),
            Direction::DOWN => Vec2::new(0, -1),
            Direction::LEFT => Vec2::new(-1, 0),
            Direction::RIGHT => Vec2::new(1, 0),
        }
    }
}

#[derive(Debug, Clone)]
struct ParseDirectionError;

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Direction::RIGHT),
            "L" => Ok(Direction::LEFT),
            "U" => Ok(Direction::UP),
            "D" => Ok(Direction::DOWN),
            _ => Err(ParseDirectionError),
        }
    }
}

pub fn parse_command(command: &str) -> (i32, Vec2) {
    let op = &command[0..1];
    let num: i32 = command[1..].parse().unwrap();
    let dir: Direction = op.parse().unwrap();
    (num, dir.into())
}

#[test]
pub fn find_closest_intersect() {
    if let Ok(lines) = read_lines("./src/year2019/data/day3input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut world: World<bool> = World::new();
        let mut min_dist = 9999999;
        for (line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                dbg!(&contents);
                if line_num == 0 {
                    let mut pos = Vec2::new(0, 0);
                    // mark first positions
                    for command in contents.split(",") {
                        let (num, dir) = parse_command(command);
                        for i in 1..=num {
                            let movement: Vec2 = dir * i;
                            world.set(pos + movement, true);
                        }
                        pos += dir * num;
                    }
                } else {
                    let mut pos = Vec2::new(0, 0);
                    // mark first positions
                    for command in contents.split(",") {
                        let (num, dir) = parse_command(command);
                        for i in 1..=num {
                            let movement: Vec2 = dir * i;
                            if *world.get(pos + movement) {
                                let dist = (pos + movement).abs().to_array().iter().sum::<i32>();
                                println!("Collision at {}, dist={}", pos + movement, dist);
                                min_dist = min_dist.min(dist);
                            }
                        }
                        pos += dir * num;
                    }
                }
            }
            dbg!(min_dist);
        }
    }
}

#[test]
pub fn find_min_sig_delay_intersect() {
    if let Ok(lines) = read_lines("./src/year2019/data/day3input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut world: World<i32> = World::new();
        let mut min_delay = 9999999;
        for (line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                dbg!(&contents);
                if line_num == 0 {
                    let mut pos = Vec2::new(0, 0);
                    let mut delay = 0;
                    // mark first positions
                    for command in contents.split(",") {
                        let (num, dir) = parse_command(command);
                        for i in 1..=num {
                            let movement: Vec2 = dir * i;
                            if *world.get(pos + movement) == 0 {
                                world.set(pos + movement, delay + i);
                            }
                        }
                        pos += dir * num;
                        delay += num;
                        dbg!(delay);
                    }
                } else {
                    let mut pos = Vec2::new(0, 0);
                    let mut delay = 0;
                    for command in contents.split(",") {
                        let (num, dir) = parse_command(command);
                        for i in 1..=num {
                            let movement: Vec2 = dir * i;
                            if *world.get(pos + movement) > 0 {
                                let other_delay = *world.get(pos + movement);
                                println!(
                                    "Collision at {}, dist={}",
                                    pos + movement,
                                    delay + other_delay + i
                                );
                                min_delay = min_delay.min(delay + other_delay + i);
                            }
                        }
                        pos += dir * num;
                        delay += num;
                    }
                }
            }
            dbg!(min_delay);
        }
    }
}
