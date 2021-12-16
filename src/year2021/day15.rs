use crate::utils::*;
use core::cmp::Ordering;
use hashbrown::{HashMap, HashSet};
use image::ImageBuffer;
use noise::{NoiseFn, Perlin, RidgedMulti};
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct WeightedPos {
    pub weight: usize,
    pub pos: Vec2,
}

impl WeightedPos {
    pub fn new(weight: usize, pos: Vec2) -> Self {
        WeightedPos { weight, pos }
    }
}

impl Ord for WeightedPos {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for WeightedPos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn ridged_noise() {
    let ridged = RidgedMulti::new();

    let mut world = HashMap::new();

    let img = ImageBuffer::from_fn(1500, 1500, |x, y| {
        let val = ridged
            .get([x as f64 / 100., y as f64 / 100.])
            .clamp(-1., 1.);
        let brightness = (((val + 1.) / 2.) * 255.) as u8;
        world.insert(
            Vec2::new(x as i32, y as i32),
            (255 - brightness) as usize * 10,
        );
        image::Luma([brightness])
    });

    img.save("./src/year2021/viz/day15/aaa.png");

    let mut map = World { world };
    find_path(&mut map, 1);
}

pub fn day_15() {
    if let Some(mut map) = World::<usize>::from_file("./src/year2021/data/day15input.txt") {
        let max_x = map.max_x() + 1;
        let max_y = map.max_y() + 1;
        let points: Vec<_> = map.world.iter().map(|(&x, &y)| (x, y)).collect();
        for (pt, weight) in points {
            let weight = weight - 1;
            for x in 0..5 {
                for y in 0..5 {
                    if x == 0 && y == 0 {
                        continue;
                    }
                    let pos = Vec2::new((x * max_x) + pt.x, (y * max_y) + pt.y);

                    map.world
                        .insert(pos, ((weight + x as usize + y as usize) % 9) + 1);
                }
            }
        }
        find_path(&mut map, 3);
    }
}

pub fn find_path(map: &mut World<usize>, px_per_node: u32) {
    let mut completed = HashSet::new();
    let mut heap: BinaryHeap<WeightedPos> = BinaryHeap::new();
    let mut distances = HashMap::new();
    heap.push(WeightedPos::new(0, Vec2::new(0, 0)));
    for pos in map.world.keys() {
        distances.insert(*pos, usize::MAX);
    }
    distances.insert(Vec2::new(0, 0), 0);

    let end = Vec2::new(map.max_x(), map.max_y());
    let mut visit_num = 0;
    let diagnostics = false;
    let mut visited_world = World {
        world: HashMap::new(),
    };
    let mut weight_world = World {
        world: HashMap::new(),
    };
    let mut heuristic_world = World {
        world: HashMap::new(),
    };

    let cost_scale = *map.world.values().min().unwrap();
    dbg!(cost_scale);

    let image_width = (map.max_x() + 1) as u32;
    let image_height = (map.max_y() + 1) as u32;
    let mut nodes_in_frame = 0;
    let nodes_per_frame = ((image_width * image_height) / (24 * 20)).max(1);
    let mut frame_num = 0;

    let mut write_frame = |distances: &HashMap<Vec2, usize>| {
        let max = (*distances
            .values()
            .filter(|&&x| x < usize::MAX)
            .max()
            .unwrap_or(&1)) as f32
            * 1.1;
        let img = ImageBuffer::from_fn(
            image_width * px_per_node,
            image_height * px_per_node,
            |x, y| {
                let pos = Vec2::new((x / px_per_node) as i32, (y / px_per_node) as i32);
                let distance = *distances.get(&pos).unwrap_or(&0);
                let distance = (distance as f32).min(max);
                let ratio = (distance as f32) / (max as f32);
                let brightness = ratio * 255.;

                image::Luma([(255. - brightness) as u8])
            },
        );
        img.save(format!("./src/year2021/viz/day15/map{:03}.png", frame_num))
            .unwrap();
        frame_num += 1;
    };

    while let Some(pos) = heap.pop() {
        if !completed.contains(&pos.pos) {
            //dbg!(&pos.pos);
            completed.insert(pos.pos);
            let weight = distances[&pos.pos];

            let diff = end - pos.pos;
            let h = (diff.x.abs() + diff.y.abs()) as usize;
            if diagnostics {
                visited_world.world.insert(pos.pos, visit_num);
                weight_world.world.insert(pos.pos, weight);
                heuristic_world.world.insert(pos.pos, h);
            }

            visit_num += 1;

            if pos.pos == end {
                break;
            }
            for neighbor in get_cardinal_neighbors(pos.pos) {
                if let Some(&neighbor_weight) = map.world.get(&neighbor) {
                    let new_weight = neighbor_weight + weight;
                    if new_weight < distances[&neighbor] {
                        let diff = end - neighbor;
                        let h = cost_scale * (diff.x.abs() + diff.y.abs()) as usize;
                        distances.insert(neighbor, new_weight);
                        heap.push(WeightedPos::new(new_weight + h, neighbor));
                    }
                }
            }

            nodes_in_frame += 1;
            if nodes_in_frame == nodes_per_frame {
                write_frame(&distances);
                nodes_in_frame = 0;
            }
        }
    }
    write_frame(&distances);
    //visited_world.pretty_print();
    dbg!((map.max_x() + 1) * (map.max_y() + 1));
    dbg!(heap.len());
    dbg!(completed.len());
    dbg!(distances[&end]);
}
