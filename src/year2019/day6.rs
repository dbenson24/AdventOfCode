use std::collections::{BTreeMap, BTreeSet};

use crate::utils::*;

#[test]
pub fn count_edges() {
    if let Ok(lines) = read_lines("./src/year2019/data/day6input.txt") {
        // Consumes the iterator, returns an (Optional) String

        let mut map: BTreeMap<String, String> = BTreeMap::new();
        for (line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                let objects: Vec<String> = contents.split(")").map(|s| s.to_owned()).collect();
                map.insert(objects[1].clone(), objects[0].clone());
            }
        }

        let mut connections = 0;
        for key in map.keys() {
            let mut parent = map.get(key);
            while let Some(p) = parent {
                connections += 1;
                parent = map.get(p);
            }
        }
        dbg!(connections);
    }
}

#[test]
pub fn find_parent() {
    if let Ok(lines) = read_lines("./src/year2019/data/day6input.txt") {
        // Consumes the iterator, returns an (Optional) String

        let mut map: BTreeMap<String, String> = BTreeMap::new();
        for (line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                let objects: Vec<String> = contents.split(")").map(|s| s.to_owned()).collect();
                map.insert(objects[1].clone(), objects[0].clone());
            }
        }
        let mut you_parents = BTreeSet::<&String>::new();

        let mut connections = 0;

        let mut parent = map.get("YOU");
        while let Some(p) = parent {
            you_parents.insert(p);
            parent = map.get(p);
        }

        let mut san_len = 0;

        dbg!(&you_parents);
        let mut parent = map.get("SAN");
        while let Some(p) = parent {
            if you_parents.contains(p) {
                break;
            } else {
                parent = map.get(p);
            }
            san_len += 1;
        }

        let target_parent = parent.unwrap();

        let mut you_len = 0;
        let mut parent = map.get("YOU");
        while let Some(p) = parent {
            if p == target_parent {
                break;
            }
            parent = map.get(p);
            you_len += 1;
        }

        dbg!(you_len + san_len);
    }
}
