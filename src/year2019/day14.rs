use std::{hash::Hash, ops::Add, slice::SliceIndex, sync::mpsc::channel};

use crate::utils::*;
use crate::year2019::intcode::*;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use rayon::spawn;

pub fn upsert<K: Eq + Hash, V: Add<Output = V> + Copy>(map: &mut HashMap<K, V>, key: K, val: V) {
    if let Some(x) = map.get_mut(&key) {
        *x = x.add(val);
    } else {
        map.insert(key, val);
    }
}

pub fn add_to<K: Eq + Hash, V: Eq + Hash>(map: &mut HashMap<K, HashSet<V>>, key: K, val: V) {
    if let Some(x) = map.get_mut(&key) {
        x.insert(val);
    } else {
        let mut set = HashSet::new();
        set.insert(val);
        map.insert(key, set);
    }
}

pub fn order_conversions<'a>(
    recipes: &'a HashMap<String, (i64, Vec<(String, i64)>)>,
    visited: &mut HashSet<&'a str>,
    curr: &'a str,
    ordered: &mut Vec<&'a str>,
) {
    if visited.contains(curr) {
        return;
    }
    visited.insert(curr);
    if let Some((_, next)) = recipes.get(curr) {
        for (x, _) in next {
            order_conversions(recipes, visited, x, ordered);
        }
    }
    ordered.push(curr);
}

pub fn day14() {
    if let Ok(lines) = read_lines("./src/year2019/data/day14input.txt") {
        // Consumes the iterator, returns an (Optional) String

        let mut recipes = HashMap::new();
        let mut dependents: HashMap<String, HashSet<String>> = HashMap::new();
        for (line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {
                let mut iter = contents.split(" => ");
                let inputs = iter.next().unwrap();
                let output = iter.next().unwrap();
                let parse_pair = |s: &str| {
                    let mut i = s.split(" ");
                    let count: i64 = i.next().unwrap().parse().unwrap();
                    let name = i.next().unwrap().to_string();
                    (name, count)
                };

                let inputs = inputs.split(", ").map(parse_pair).collect_vec();
                let output = parse_pair(output);

                for (input, _) in &inputs {
                    add_to(&mut dependents, input.clone(), output.0.clone());
                }
                recipes.insert(output.0, (output.1, inputs));
            }
        }

        let mut conversion_order = vec![];
        let mut visited: HashSet<&str> = HashSet::new();
        order_conversions(&recipes, &mut visited, "FUEL", &mut conversion_order);

        conversion_order.reverse();
        conversion_order.truncate(conversion_order.len() - 1);

        let mut inventory = HashMap::new();

        let mut i = 0;
        loop {
            upsert(&mut inventory, "FUEL", 1i64);
            for &item in &conversion_order {
                let mut curr_amt = *inventory.get(item).unwrap_or(&0);
                if let Some((amt, inputs)) = recipes.get(item) {
                    while curr_amt > 0 {
                        curr_amt -= *amt;
                        for (input, quant) in inputs {
                            upsert(&mut inventory, input, *quant);
                        }
                    }
                    inventory.insert(item, curr_amt);
                }
            }
            if inventory["ORE"] > 1000000000000i64 {
                break;
            }
            i += 1;
        }
        dbg!(i);

        let mut inventory = HashMap::new();
        inventory.insert("ORE", 1000000000000i64);
    }
}
