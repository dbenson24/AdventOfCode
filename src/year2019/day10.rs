use std::cmp::Reverse;

use crate::utils::*;
use hashbrown::{HashMap, HashSet};

#[test]
pub fn base() {
    if let Ok(lines) = read_lines("./src/year2019/data/day10input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut asteroids = HashSet::new();
        for (y, line) in lines.enumerate() {
            if let Ok(contents) = line {
                for (x, char) in contents.split("").filter(|s| s.len() > 0).enumerate() {
                    match char {
                        "#" => {
                            asteroids.insert(IVec2::new(x as i32, y as i32));
                        }
                        _ => (),
                    }
                }
            }
        }

        pretty_print_set(&asteroids, &|_| "#".to_string(), 1);

        let max = asteroids
            .iter()
            .map(|a| {
                let a_pos = a.as_vec2();
                let mut dict: HashMap<IVec2, IVec2> = HashMap::new();
                for (pos, dir) in asteroids
                    .iter()
                    .filter(|&b| b != a)
                    .map(|b| (*b, b.as_vec2()))
                    .map(|(pos, b)| (pos, ((a_pos - b).normalize() * 10000.).round().as_ivec2()))
                {
                    if let Some(other_pos) = dict.get(&dir) {
                        let dist_a = (*a - *other_pos).abs();
                        let dist_b = (*a - pos).abs();
                        if dist_a.x + dist_a.y > dist_b.x + dist_b.y {
                            dict.insert(dir, pos);
                        }
                    } else {
                        dict.insert(dir, pos);
                    }
                }
                (a, dict)
            })
            .max_by(|x, y| x.1.len().cmp(&y.1.len()))
            .unwrap();
        dbg!(max.0, max.1.len());
        let _asteroid = max.0.as_vec2();
        let mut asteroids_to_destroy: Vec<_> = max
            .1
            .iter()
            .map(|(dir, pos)| {
                let dir = dir.as_vec2().normalize();
                let rads = 0. - f32::atan2(dir.x, dir.y); // - (3.14159 / 2.);
                if *pos == IVec2::new(11, 12) {
                    dbg!(rads);
                }
                if *pos == IVec2::new(12, 1) {
                    dbg!(rads);
                }
                let rads = (2. * 3.14159) + rads;
                let rads = rads % (2. * 3.14159);
                //dbg!(rads);
                //let rads = (rads - (3.14159 / 2.) ) % (2. * 3.14159);
                (rads, pos)
            })
            .collect();
        asteroids_to_destroy.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap());
        //dbg!(&asteroids_to_destroy[0..20]);
        dbg!(asteroids_to_destroy[199]);
        let _station = *max.0;
    }
}
