use std::collections::{HashMap, HashSet};

use crate::utils::*;

static START: &str = "start";
static END: &str = "end";

#[test]
pub fn day_12_part_1() {
    if let Ok(lines) = read_lines("./src/year2021/data/day12testinput.txt") {
        let mut edges: HashMap<String, Vec<String>> = HashMap::new();
        // Consumes the iterator, returns an (Optional) String
        for (line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                let edge: Vec<_> = contents.split("-").collect();
                if let Some(vec) = edges.get_mut(edge[0]) {
                    vec.push(edge[1].to_string());
                } else {
                    edges.insert(edge[0].to_string(), vec![edge[1].to_string()]);
                }

                if let Some(vec) = edges.get_mut(edge[1]) {
                    vec.push(edge[0].to_string());
                } else {
                    edges.insert(edge[1].to_string(), vec![edge[0].to_string()]);
                }
            }
        }

        let mut paths = 0;
        let starting_string = START.to_owned();
        let mut stack = vec![(&starting_string, HashSet::<&String>::new())];
        while let Some((cave, mut curr_small_caves)) = stack.pop() {
            if let Some(connected) = edges.get(cave) {
                if cave.as_bytes()[0].is_ascii_lowercase() {
                    curr_small_caves.insert(cave);
                }
                for dest in connected {
                    if dest == END {
                        paths += 1;
                    } else {
                        if !(dest.as_bytes()[0].is_ascii_lowercase()
                            && curr_small_caves.contains(dest))
                        {
                            stack.push((dest, curr_small_caves.clone()));
                        }
                    }
                }
            }
        }

        //dbg!(edges);

        dbg!(paths);
    }
}

#[test]
pub fn day_12_part_2() {
    if let Ok(lines) = read_lines("./src/year2021/data/day12testinput.txt") {
        let mut edges: HashMap<String, Vec<String>> = HashMap::new();
        // Consumes the iterator, returns an (Optional) String
        for (line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                let edge: Vec<_> = contents.split("-").collect();
                if let Some(vec) = edges.get_mut(edge[0]) {
                    vec.push(edge[1].to_string());
                } else {
                    edges.insert(edge[0].to_string(), vec![edge[1].to_string()]);
                }

                if let Some(vec) = edges.get_mut(edge[1]) {
                    vec.push(edge[0].to_string());
                } else {
                    edges.insert(edge[1].to_string(), vec![edge[0].to_string()]);
                }
            }
        }

        let mut paths = 0;
        let starting_string = START.to_owned();
        let mut stack = vec![(&starting_string, HashMap::<&String, i32>::new())];
        while let Some((cave, mut curr_small_caves)) = stack.pop() {
            if let Some(connected) = edges.get(cave) {
                if cave.as_bytes()[0].is_ascii_lowercase() {
                    if let Some(visits) = curr_small_caves.get(cave) {
                        curr_small_caves.insert(cave, visits + 1);
                    } else {
                        curr_small_caves.insert(cave, 1);
                    }
                }
                for dest in connected {
                    if dest == END {
                        paths += 1;
                    } else {
                        if dest.as_bytes()[0].is_ascii_lowercase()
                            && curr_small_caves.contains_key(dest)
                        {
                            if curr_small_caves.values().all(|x| *x == 1)
                                && dest != START
                                && dest != END
                            {
                                stack.push((dest, curr_small_caves.clone()));
                            }
                        } else {
                            stack.push((dest, curr_small_caves.clone()));
                        }
                    }
                }
            }
        }

        //dbg!(edges);

        dbg!(paths);
    }
}
