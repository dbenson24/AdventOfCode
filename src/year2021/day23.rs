use crate::utils::*;
use core::cmp::Ordering;
use hashbrown::{HashMap, HashSet};
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Amphipod {
    pub stopx: i32,
    pub pos: IVec2,
    pub cost: i32,
}

impl Amphipod {
    pub fn skip_space(&self, pos: IVec2) -> bool {
        if pos.x.abs() == 1 || pos.x.abs() == 3 {
            pos.x == self.stopx
        } else {
            false
        }
    }

    pub fn new(char: &str, x: i32, y: i32) -> Amphipod {
        let pos = IVec2::new(x, y);
        let (stopx, cost) = match char {
            "A" => (-3, 1),
            "B" => (-1, 10),
            "C" => (1, 100),
            _ => (3, 1000),
        };
        Amphipod { pos, stopx, cost }
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
    pub state: State,
}

impl WeightedPos {
    pub fn new(weight: i32, state: State) -> Self {
        WeightedPos { weight, state }
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
            || pos.y > 1
            || (pos.y < 0 && !(pos.x.abs() == 1 || pos.x.abs() == 3))
            || pos.y < -2
    }

    pub fn move_pod(&self, pod_i: usize, dir: Dir2) -> Option<(i32, State)> {
        let pod = self.pods[pod_i];
        let mut dist = -1;
        let mut dst: IVec2 = pod.pos;
        dbg!(pod.pos);

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
    }

    pub fn moves(&self) -> Vec<(i32, State)> {
        let mut moves = vec![];
        for (i, pod) in self.pods.iter().enumerate() {
            if pod.pos.x == pod.stopx {
                if self
                    .pods
                    .iter()
                    .all(|x| x.stopx == pod.stopx && x.pos.x == x.stopx && x.pos.y < 0)
                {
                    continue;
                }
            }

            if let Some(last) = self.last {
                if last != i && pod.pos.y == 0 {
                    continue;
                }
            }

            if let Some(state) = self.move_pod(i, Dir2::Left) {
                moves.push(state);
            }

            if let Some(state) = self.move_pod(i, Dir2::Right) {
                moves.push(state);
            }
        }
        moves
    }
}

#[test]
pub fn day_23() {
    let mut state = State {
        pods: vec![
            Amphipod::new("B", -3, -1),
            Amphipod::new("A", -3, -2),
            Amphipod::new("C", -1, -1),
            Amphipod::new("D", -1, -2),
            Amphipod::new("B", 1, -1),
            Amphipod::new("C", 1, -2),
            Amphipod::new("D", 3, -1),
            Amphipod::new("A", 3, -2),
        ],
        last: None,
    };

    find_path(&mut state);
}

pub fn find_path(state: &mut State) {
    let mut completed = HashSet::new();
    let mut heap: BinaryHeap<WeightedPos> = BinaryHeap::new();
    heap.push(WeightedPos::new(0, state.clone()));

    let mut visit_num = 0;
    let diagnostics = false;
    let mut cost = 0;

    while let Some(pos) = heap.pop() {
        if !completed.contains(&pos.state) {
            completed.insert(pos.state.clone());

            visit_num += 1;
            dbg!(visit_num);

            if pos.state.done() {
                cost = pos.weight;
                break;
            }
            for (w, s) in pos.state.moves() {
                heap.push(WeightedPos::new(w, s));
            }
        }
    }
    dbg!(heap.len());
    dbg!(completed.len());
    dbg!(cost);
}
