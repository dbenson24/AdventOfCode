use crate::utils::*;
use core::cmp::Ordering;
use hashbrown::{HashMap, HashSet};
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Amphipod {
    pub stopx: i32,
    pub startx: i32,
    pub pos: IVec2,
    pub cost: i32,
    pub frozen: bool,
}

// const POSITIONS: [IVec2; 27] = [
//     IVec2::new(
//         -5,
//         0,
//     ),
//     IVec2::new(
//         -4,
//         0,
//     ),
//     IVec2::new(
//         -3,
//         0,
//     ),
//     IVec2::new(
//         -2,
//         0,
//     ),
//     IVec2::new(
//         -1,
//         0,
//     ),
//     IVec2::new(
//         0,
//         0,
//     ),
//     IVec2::new(
//         1,
//         0,
//     ),
//     IVec2::new(
//         2,
//         0,
//     ),
//     IVec2::new(
//         3,
//         0,
//     ),
//     IVec2::new(
//         4,
//         0,
//     ),
//     IVec2::new(
//         5,
//         0,
//     ),
//     IVec2::new(
//         -3,
//         -4,
//     ),
//     IVec2::new(
//         -3,
//         -3,
//     ),
//     IVec2::new(
//         -3,
//         -2,
//     ),
//     IVec2::new(
//         -3,
//         -1,
//     ),
//     IVec2::new(
//         -1,
//         -4,
//     ),
//     IVec2::new(
//         -1,
//         -3,
//     ),
//     IVec2::new(
//         -1,
//         -2,
//     ),
//     IVec2::new(
//         -1,
//         -1,
//     ),
//     IVec2::new(
//         1,
//         -4,
//     ),
//     IVec2::new(
//         1,
//         -3,
//     ),
//     IVec2::new(
//         1,
//         -2,
//     ),
//     IVec2::new(
//         1,
//         -1,
//     ),
//     IVec2::new(
//         3,
//         -4,
//     ),
//     IVec2::new(
//         3,
//         -3,
//     ),
//     IVec2::new(
//         3,
//         -2,
//     ),
//     IVec2::new(
//         3,
//         -1,
//     ),
// ];

impl Amphipod {
    pub fn skip_space(&self) -> bool {
        self.pos.y == 0 && (self.pos.x.abs() == 1 || self.pos.x.abs() == 3)
    }

    pub fn new(char: &str, x: i32, y: i32) -> Amphipod {
        let pos = IVec2::new(x, y);
        let startx = x;
        let (stopx, cost) = match char {
            "A" => (-3, 1),
            "B" => (-1, 10),
            "C" => (1, 100),
            _ => (3, 1000),
        };
        Amphipod { pos, stopx, cost, startx, frozen: false }
    }

    pub fn get_string(&self) -> String {
        let letter = match self.cost {
            1 => "A",
            10 => "B",
            100 => "C",
            _ => "D"
        }.to_string();
        if self.frozen {
            letter.to_ascii_lowercase()
        } else {
            letter
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State {
    pub pods: Vec<Amphipod>,
    pub last: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WeightedPos {
    pub weight: i32,
    pub cost: i32,
    pub state: State,
}

impl WeightedPos {
    pub fn new(weight: i32, cost: i32, state: State) -> Self {
        WeightedPos { weight, cost, state }
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

impl State {
    pub fn done(&self) -> bool {
        self.pods.iter().all(|a| a.pos.y < 0 && a.pos.x == a.stopx)
    }

    pub fn blocked(&self, pos: IVec2) -> bool {
        self.pods.iter().any(|pod| pod.pos == pos)
            || pos.y == 0 && pos.x.abs() > 5
            || pos.y > 0
            || (pos.y < 0 && !(pos.x.abs() == 1 || pos.x.abs() == 3))
            || pos.y < -4
    }

    pub fn invalid(&self, pod: &Amphipod) -> bool {
        if pod.startx == pod.pos.x {
            return false;
        }
        if pod.pos.y < 0 {
            if !(pod.pos.x == pod.stopx || pod.pos.x == pod.startx) {
                return true;
            }
            for other in &self.pods {
                if other.pos.y < 0 && other.pos.x == pod.pos.x && other.cost != pod.cost {
                    //dbg!(&other, &pod);
                    return true;
                }
            }
        }
        false
    }

    pub fn move_pod(&self, pod_i: usize, dir: Dir2) -> Option<(i32, State)> {
        let pod = self.pods[pod_i];
        let offset: IVec2 = dir.into();
        let dest = pod.pos + offset;
        if !self.blocked(dest) {
            let mut copy = self.clone();
            copy.pods[pod_i].pos = dest;
            copy.last = if dest.y == 0 { 
                copy.pods[pod_i].startx = -100;
                for (i, pod) in copy.pods.iter_mut().enumerate() {
                    if i != pod_i && pod.pos.y == 0 {
                        pod.frozen = true;
                    }
                }
                Some(pod_i) 
            } else { 
                None 
            };
            //dbg!(pod.pos, dst, dist);
            if copy.invalid(&copy.pods[pod_i]) {
                //copy.pretty_print();
                //println!("{}", copy.pods[pod_i].get_string());
                None
            } else {
                //dbg!(dest);
                Some((pod.cost, copy))
            }
        } else {
            None
        }
        /* 
        match dir {
            Dir2::Up => None,
            Dir2::Down => None,
            _ => {
                dbg!(dir);
                let mut mvmt = IVec2::ZERO;
                while dst + mvmt == pod.pos
                    || (pod.skip_space(dst + mvmt) || pod.stopx == dst.x || dst.x < 0)
                        && !self.blocked(dst + mvmt)
                {
                    dist += 1;
                    dst += mvmt;
                    //dbg!(dst);
                    mvmt = if dst.x == pod.stopx {
                        Dir2::Down.into()
                    } else if dst.y < 0 {
                        Dir2::Up.into()
                    } else {
                        dir.into()
                    };
                    //dbg!(mvmt, self.blocked(dst + mvmt));
                }
                //dbg!(dst);
                if !self.blocked(dst) && dist > 0 {
                    let mut copy = self.clone();
                    copy.pods[pod_i].pos = dst;
                    copy.last = if dst.y == 0 { Some(pod_i) } else { None };
                    //dbg!(pod.pos, dst, dist);
                    Some((pod.cost * dist, copy))
                } else {
                    None
                }
            }
        }
        */
    }

    pub fn moves(&self) -> Vec<(i32, State)> {
        let mut moves = vec![];
        for (i, pod) in self.pods.iter().enumerate() {
            if pod.pos.x == pod.stopx {
                if self
                    .pods
                    .iter()
                    .filter(|x| x.stopx == pod.stopx)
                    .all(|x| x.pos.x == x.stopx && x.pos.y < 0)
                {
                    continue;
                }
            }
            /* 
            if let Some(last) = self.last {
                if last != i && pod.pos.y == 0 {
                    continue;
                }
            }
            */
            if pod.frozen {
                if self.pods.iter().filter(|x| x.pos.x == pod.stopx).all(|x| x.pos.x == x.stopx) {
                    let mut cost = 0;
                    let mut state = self.clone();
                    let diff = pod.stopx - pod.pos.x;
                    let dir = if diff > 0 {
                        Dir2::Right
                    } else {
                        Dir2::Left
                    };
                    while let Some(next_state) = state.move_pod(i, dir) {
                        cost += next_state.0;
                        state = next_state.1;
                        if state.pods[i].pos.x == pod.stopx {
                            break;
                        }
                    }
                    if state.pods[i].pos.x == pod.stopx {
                        while let Some(next_state) = state.move_pod(i, Dir2::Down) {
                            cost += next_state.0;
                            state = next_state.1;
                        }
                        if state.pods[i].pos.y < 0 {
                            moves.push((cost, state))
                        }
                    }
                }
                continue;
            }

            if let Some(mut state) = self.move_pod(i, Dir2::Left) {
                if state.1.pods[i].skip_space() {
                    if let Some(mut state_2) = state.1.move_pod(i, Dir2::Left) {
                        state_2.0 += state.0;
                        moves.push(state_2);
                    }
                    if let Some(mut state_2) = state.1.move_pod(i, Dir2::Down) {
                        state_2.0 += state.0;
                        moves.push(state_2);
                    }
                } else {
                    moves.push(state);
                }
            }
            if let Some(mut state) = self.move_pod(i, Dir2::Right) {
                if state.1.pods[i].skip_space() {
                    if let Some(mut state_2) = state.1.move_pod(i, Dir2::Right) {
                        state_2.0 += state.0;
                        moves.push(state_2);
                    }
                    if let Some(mut state_2) = state.1.move_pod(i, Dir2::Down) {
                        state_2.0 += state.0;
                        moves.push(state_2);
                    }
                } else {
                    moves.push(state);
                }
            }
            let mut up_cost = 0;
            let mut up_state = self.clone();
            while let Some((cost, next_state)) = up_state.move_pod(i, Dir2::Up) {
                up_cost += cost;
                up_state = next_state;
            }
            if up_cost > 0 && up_state.pods[i].skip_space() {
                if let Some(mut state_2) = up_state.move_pod(i, Dir2::Left) {
                    state_2.0 += up_cost;
                    moves.push(state_2);
                }
                if let Some(mut state_2) = up_state.move_pod(i, Dir2::Right) {
                    state_2.0 += up_cost;
                    moves.push(state_2);
                }
            }
            if let Some(mut state) = self.move_pod(i, Dir2::Down) {
                moves.push(state);
            }
        }
        moves
    }

    pub fn calc_min_cost(&self) -> i32 {
        self.pods.iter().map(|p| {
            if p.stopx == p.pos.x {
                return p.cost * (2 - p.pos.y)
            }
            let mut cost = p.cost * 2;
            if p.pos.y < 0 {
                cost += p.pos.y.abs() * p.cost;
            }
            let diff = p.startx - p.pos.x;
            cost += diff.abs() * p.cost;
            cost
        }).sum()
    }

    pub fn pretty_print(&self) {
        let mut world = World {
            world: HashMap::new()
        };

        for pod in &self.pods {
            world.world.insert(pod.pos, pod.get_string());
        }
        for x in -6..=6 {
            world.world.insert(IVec2::new(x, 1), "#".to_string());
        }

        for x in -6..=6i32 {
            if x.abs() != 1 && x.abs() != 3 {
                for y in -2..=-1 {
                    world.world.insert(IVec2::new(x, y), "#".to_string());
                }
            }
        }
        if let Some(i) = self.last {
            dbg!(self.pods[i]);
        }
        println!("======");
        world.pretty_print(&|x| {
            x.clone()
        });
    }
}

pub fn day23_example() {
    let mut pos = vec![];
    for x in -5..=5 {
        pos.push(IVec2::new(x, 0));
    }
    for x in [-3, -1, 1, 3] {
        for y in -4..=-1 {
            pos.push(IVec2::new(x, y));
        }
    }
    dbg!(pos);



    return;



    let mut state = State {
        pods: vec![
            Amphipod::new("B", -3, -1),
            Amphipod::new("D", -3, -2),
            Amphipod::new("D", -3, -3),
            Amphipod::new("A", -3, -4),
            Amphipod::new("C", -1, -1),
            Amphipod::new("C", -1, -2),
            Amphipod::new("B", -1, -3),
            Amphipod::new("D", -1, -4),
            Amphipod::new("B", 1, -1),
            Amphipod::new("B", 1, -2),
            Amphipod::new("A", 1, -3),
            Amphipod::new("C", 1, -4),
            Amphipod::new("D", 3, -1),
            Amphipod::new("A", 3, -2),
            Amphipod::new("C", 3, -3),
            Amphipod::new("A", 3, -4),
        ],
        last: None,
    };

    state.pretty_print();

    find_path(&mut state);
}

pub fn day23() {
    let mut state = State {
        pods: vec![
            Amphipod::new("B", -3, -1),
            Amphipod::new("D", -3, -2),
            Amphipod::new("D", -3, -3),
            Amphipod::new("B", -3, -4),
            Amphipod::new("C", -1, -1),
            Amphipod::new("C", -1, -2),
            Amphipod::new("B", -1, -3),
            Amphipod::new("C", -1, -4),
            Amphipod::new("A", 1, -1),
            Amphipod::new("B", 1, -2),
            Amphipod::new("A", 1, -3),
            Amphipod::new("D", 1, -4),
            Amphipod::new("D", 3, -1),
            Amphipod::new("A", 3, -2),
            Amphipod::new("C", 3, -3),
            Amphipod::new("A", 3, -4),
        ],
        last: None,
    };

    state.pretty_print();

    find_path(&mut state);
}

pub fn find_path(state: &mut State) {
    let mut completed = HashSet::new();
    let mut heap: BinaryHeap<WeightedPos> = BinaryHeap::new();
    heap.push(WeightedPos::new(0, 0, state.clone()));

    let mut visit_num = 0;
    let diagnostics = false;
    let mut cost = 0;

    while let Some(pos) = heap.pop() {
        if !completed.contains(&pos.state) {
            completed.insert(pos.state.clone());

            visit_num += 1;
            if visit_num % 50000 == 0{
                pos.state.pretty_print();
                dbg!(visit_num, pos.cost);
            }

            if pos.state.done() {
                cost = pos.cost;
                break;
            }
            for (cost, s) in pos.state.moves() {
                let cost = pos.cost + cost;
                heap.push(WeightedPos::new(cost, cost, s));
            }
        }
        if heap.len() == 0 {
            pos.state.pretty_print();
        }
    }

    // for mov in &heap {
    //     mov.state.pretty_print();
    //     dbg!(mov.cost);
    //     dbg!(mov.weight);
    // }
    dbg!(heap.len());
    dbg!(completed.len());
    dbg!(cost);
}
