use crate::utils::*;
use glam::Mat4;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use rayon::iter::*;
use std::{cell::RefCell, process::Output, rc::Rc, str::FromStr};

#[derive(Debug, Clone)]
pub struct Scanner {
    pub beacons: Vec<Vec3>,
}

pub fn to_hash(x: f32) -> i32 {
    (x * 100.).round() as i32
}

pub fn dist_to(p: Vec3) -> impl Fn(&Vec3) -> f32 {
    move |other: &Vec3| other.distance(p)
}

pub fn manhattan_dist(a: IVec3, b: IVec3) -> i32 {
    let diff = (a - b).abs();
    diff.x + diff.y + diff.z
}

pub fn day_19() {
    let mut rotations: Vec<Quat> = vec![];
    for fwd in 0..6 {
        for up in 0..6 {
            let fwd: Dir3 = FromPrimitive::from_i32(fwd).unwrap();
            let up: Dir3 = FromPrimitive::from_i32(up).unwrap();
            if fwd != up && fwd.flipped() != up {
                let mat = Mat4::look_at_lh(Vec3::ZERO, fwd.into(), up.into());
                let (_, quat, _) = mat.to_scale_rotation_translation();
                rotations.push(quat);
            }
        }
    }

    if let Ok(lines) = read_lines("./src/year2021/data/day19testinput.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut scanners = vec![];
        let mut beacons = None;
        for (line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                if contents.contains("---") {
                    if let Some(b) = beacons {
                        scanners.push(Scanner { beacons: b });
                    }
                    beacons = Some(vec![])
                } else {
                    let vals: Vec<f32> = contents
                        .split(",")
                        .filter(|s| s.len() > 0)
                        .map(|s| s.parse().unwrap())
                        .collect();
                    if vals.len() == 3 {
                        if let Some(beacons) = &mut beacons {
                            beacons.push(Vec3::new(vals[0], vals[1], vals[2]));
                        }
                    }
                }
            }
        }
        let beacons = beacons.unwrap();
        scanners.push(Scanner { beacons });

        /*
        Alternative Way - Match distances to pairs of points present in the dictionaries
        let dist_map: Vec<HashMap<i32, (Vec3, Vec3)>> = scanners.par_iter()
            .map(|scanner| scanner.beacons.iter()
                .permutations(2)
                .map(|xs| (to_hash(xs[0].distance(*xs[1])), (*xs[0], *xs[1]))).collect()).collect();


        let overlap = dist_map[1].iter().filter(|a| dist_map[0].contains_key(a.0)).count();
        dbg!(overlap);
        */

        let scanner0 = scanners[0].clone();
        let mut scanners: Vec<_> = scanners[1..].into_iter().collect();
        let mut scanner_positions = vec![];
        scanner_positions.push(IVec3::new(0, 0, 0));

        let mut positioned_beacons: HashSet<IVec3> =
            scanner0.beacons.iter().map(|x| x.as_ivec3()).collect();
        while scanners.len() > 0 {
            scanners = scanners
                .into_iter()
                .filter(|&scanner| {
                    let origin_and_pos = scanner
                        .beacons
                        .par_iter()
                        .fold(
                            || None,
                            |acc, beacon| {
                                if acc.is_some() {
                                    return acc;
                                }
                                let dist = dist_to(*beacon);
                                let distances: HashMap<_, _> = scanner
                                    .beacons
                                    .iter()
                                    .map(|p| (to_hash(dist(p)), *p))
                                    .collect();

                                let mut origin_and_pos = None;

                                for good_pos in &positioned_beacons {
                                    let good_dist = dist_to(good_pos.as_vec3());
                                    let mut good_distances: HashMap<_, _> = positioned_beacons
                                        .iter()
                                        .map(|p| (to_hash(good_dist(&p.as_vec3())), p.as_vec3()))
                                        .collect();

                                    let pairs: Vec<(Vec3, Vec3)> = good_distances
                                        .iter()
                                        .map(|(d, abs_pos)| {
                                            if let Some(rel_pos) = distances.get(d) {
                                                Some((*rel_pos, *abs_pos))
                                            } else {
                                                None
                                            }
                                        })
                                        .filter(|x| x.is_some())
                                        .map(|x| x.unwrap())
                                        .collect();

                                    if pairs.len() >= 12 {
                                        if let Some((origin, rot)) =
                                            get_origin_and_rotation(&pairs, &rotations)
                                        {
                                            origin_and_pos = Some((origin, rot));
                                            break;
                                        }
                                    }
                                }
                                origin_and_pos
                            },
                        )
                        .find_any(|x| x.is_some());

                    if let Some(res) = origin_and_pos {
                        if let Some((origin, rot)) = res {
                            scanner_positions.push(origin.round().as_ivec3());
                            for rel_pos in &scanner.beacons {
                                let abs_pos = (rot.mul_vec3(*rel_pos) + origin).round();
                                positioned_beacons.insert(abs_pos.as_ivec3());
                            }
                            return false;
                        }
                    }
                    true
                })
                .collect();
        }

        let highest = scanner_positions
            .iter()
            .permutations(2)
            .max_by(|xs, ys| manhattan_dist(*xs[0], *xs[1]).cmp(&manhattan_dist(*ys[0], *ys[1])))
            .unwrap();
        dbg!(&highest);
        dbg!(manhattan_dist(*highest[0], *highest[1]));
        dbg!(positioned_beacons.len());
    }
}

#[test]
pub fn base() {
    day_19();
}

pub fn get_origin_and_rotation(
    positions: &[(Vec3, Vec3)],
    rotations: &[Quat],
) -> Option<(Vec3, Quat)> {
    for rot in rotations {
        let start = positions[0];
        let origin = start.1 - rot.mul_vec3(start.0);
        let mut valid = true;
        for pos in positions {
            let rel_pos = rot.mul_vec3(pos.0) + origin;
            if rel_pos.distance(pos.1) > 0.01 {
                valid = false;
                break;
            }
        }
        if valid {
            return Some((origin, *rot));
        }
    }
    return None;
}
