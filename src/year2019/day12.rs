use crate::utils::*;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

fn gcd(x: usize, y: usize) -> usize {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn lcm3(a: usize, b: usize, c: usize) -> usize {
    lcm(a, lcm(b, c))
}

pub fn day12() {
    let mut _test_moons = [
        IVec3::new(-1, 0, 2),
        IVec3::new(2, -10, -7),
        IVec3::new(4, -8, 8),
        IVec3::new(3, 5, -1),
    ];
    let mut moons = [
        IVec3::new(-8, -18, 6),
        IVec3::new(-11, -14, 4),
        IVec3::new(8, -3, -10),
        IVec3::new(-2, -16, 1),
    ];

    let mut vels = [IVec3::ZERO; 4];
    let mut x_states = HashMap::new();
    let mut y_states = HashMap::new();
    let mut z_states = HashMap::new();

    let mut x_period = None;
    let mut y_period = None;
    let mut z_period = None;

    let mut i = 0usize;
    loop {
        if x_period.is_none() {
            let x_state = moons
                .iter()
                .zip(vels.iter())
                .map(|(pos, vel)| (pos.x, vel.x))
                .collect_vec();
            if let Some(start) = x_states.insert(x_state, i) {
                x_period = Some((start, i - start));
            }
        }

        if y_period.is_none() {
            let y_state = moons
                .iter()
                .zip(vels.iter())
                .map(|(pos, vel)| (pos.y, vel.y))
                .collect_vec();
            if let Some(start) = y_states.insert(y_state, i) {
                y_period = Some((start, i - start));
            }
        }

        if z_period.is_none() {
            let z_state = moons
                .iter()
                .zip(vels.iter())
                .map(|(pos, vel)| (pos.z, vel.z))
                .collect_vec();
            if let Some(start) = z_states.insert(z_state, i) {
                z_period = Some((start, i - start));
            }
        }

        if x_period.is_some() && y_period.is_some() && z_period.is_some() {
            break;
        }

        for (i, vel) in vels.iter_mut().enumerate() {
            let curr_pos = moons[i];
            for (j, &pos) in moons.iter().enumerate() {
                if j != i {
                    let gravity = (pos - curr_pos).signum();
                    *vel = *vel + gravity;
                }
            }
        }
        for i in 0..4 {
            moons[i] = moons[i] + vels[i];
        }

        i += 1;
    }
    dbg!(i);
    let energy = |x: &IVec3| {
        let x = x.abs();
        x.x + x.y + x.z
    };
    let e: i32 = moons
        .iter()
        .zip(vels.iter())
        .map(|(pos, vel)| energy(pos) * energy(vel))
        .sum();
    let x_period = x_period.unwrap();
    let y_period = y_period.unwrap();
    let z_period = z_period.unwrap();
    dbg!(x_period, y_period, z_period);

    dbg!(lcm3(x_period.1, y_period.1, z_period.1));
    dbg!(e);
}
