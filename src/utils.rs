pub use glam::{IVec2, IVec3, Quat, Vec2, Vec3};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::*;
use pad::PadStr;
pub use smallvec::{smallvec, SmallVec};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Clone)]
pub struct VecWorld<T: Default> {
    data: Vec<Vec<T>>,
}

impl<T: Default + Clone> VecWorld<T> {
    pub fn new() -> Self {
        VecWorld {
            data: vec![vec![T::default(); 50000]; 50000],
        }
    }

    pub fn get(&self, pos: IVec2) -> &T {
        &self.data[(pos.x + 25000) as usize][(pos.y + 25000) as usize]
    }

    pub fn set(&mut self, pos: IVec2, val: T) {
        self.data[(pos.x + 25000) as usize][(pos.y + 25000) as usize] = val;
    }
}

pub fn get_neighbors(pos: IVec2) -> [IVec2; 8] {
    let x = pos.x;
    let y = pos.y;
    let mut i = 0;
    let mut neighs: [IVec2; 8] = Default::default();
    for y_i in y - 1..y + 2 {
        for x_i in x - 1..x + 2 {
            if !(x == x_i && y == y_i) {
                neighs[i] = IVec2::new(x_i, y_i);
                i += 1;
            }
        }
    }
    neighs
}

pub fn get_neighbors_and_pos(pos: IVec2) -> [IVec2; 9] {
    let x = pos.x;
    let y = pos.y;
    let mut i = 0;
    let mut neighs: [IVec2; 9] = Default::default();
    for y_i in y - 1..y + 2 {
        for x_i in x - 1..x + 2 {
            neighs[i] = IVec2::new(x_i, y_i);
            i += 1;
        }
    }
    neighs
}

pub fn get_cardinal_neighbors(pos: IVec2) -> [IVec2; 4] {
    let x = pos.x;
    let y = pos.y;

    [
        IVec2::new(x - 1, y),
        IVec2::new(x + 1, y),
        IVec2::new(x, y - 1),
        IVec2::new(x, y + 1),
    ]
}
pub struct World<T> {
    pub world: HashMap<IVec2, T>,
}

impl<T> World<T> {
    pub fn from_file<U: Into<Option<T>>>(
        path: &str,
        parse_fn: &impl Fn(&str) -> U,
    ) -> Option<Self> {
        if let Ok(lines) = read_lines(path) {
            let mut world = HashMap::new();
            for (y, line) in lines.enumerate() {
                if let Ok(contents) = line {
                    for (x, height) in contents
                        .split("")
                        .filter(|s| s.len() > 0)
                        .map(parse_fn)
                        .map(|x| {
                            let x: Option<T> = x.into();
                            x
                        })
                        .enumerate()
                        .filter(|(i, x)| x.is_some())
                        .map(|(i, x)| (i, x.unwrap()))
                    {
                        let pos = IVec2::new(x as i32, y as i32);
                        world.insert(pos, height);
                    }
                }
            }
            Some(World { world })
        } else {
            None
        }
    }

    pub fn pretty_print(&self, str_fn: &impl Fn(&T, IVec2) -> String, rev_y: bool) {
        let max_y = self.max_y();
        let max_x = self.max_x();
        let min_x = self.min_x().min(0);
        let min_y = self.min_y().min(0);

        if rev_y {
            for y in (min_y..=max_y).rev() {
                let mut row = vec![];
                for x in min_x..=max_x {
                    let pos = IVec2::new(x, y);
                    if let Some(val) = self.world.get(&pos) {
                        row.push(str_fn(val, pos));
                    } else {
                        row.push(" ".to_string());
                    }
                }
                println!("{}", row.iter().join(""));
            }
        } else {
            for y in min_y..=max_y {
                let mut row = vec![];
                for x in min_x..=max_x {
                    let pos = IVec2::new(x, y);
                    if let Some(val) = self.world.get(&pos) {
                        row.push(str_fn(val, pos));
                    } else {
                        row.push(" ".to_string());
                    }
                }
                println!("{}", row.iter().join(""));
            }
        }
    }

    pub fn max_x(&self) -> i32 {
        self.world.keys().map(|pos| pos.x).max().unwrap()
    }
    pub fn max_y(&self) -> i32 {
        self.world.keys().map(|pos| pos.y).max().unwrap()
    }

    pub fn min_x(&self) -> i32 {
        self.world.keys().map(|pos| pos.x).min().unwrap()
    }
    pub fn min_y(&self) -> i32 {
        self.world.keys().map(|pos| pos.y).min().unwrap()
    }

    pub fn min_cost_4<S>(
        &mut self,
        state: &mut S,
        start: IVec2,
        done: impl Fn(IVec2, &mut Self, &mut S) -> bool,
        get_weight: impl Fn(IVec2, &mut Self, &mut S) -> Option<usize>,
    ) -> Option<usize> {
        djikstra(
            start,
            0,
            &mut (self, state),
            |&pos, _, (world, extra_state)| done(pos, world, extra_state),
            |pos, _, (world, extra_state)| {
                let x: SmallVec<[_; 4]> = get_cardinal_neighbors(*pos)
                    .iter()
                    .map(|x| {
                        if let Some(w) = get_weight(*x, world, extra_state) {
                            Some((*x, w))
                        } else {
                            None
                        }
                    })
                    .collect();
                x
            },
        )
    }

    pub fn min_path_4<S>(
        &mut self,
        extra_state: &mut S,
        start: IVec2,
        done: impl Fn(IVec2, &mut Self, &mut S) -> bool,
        get_weight: impl Fn(IVec2, &mut Self, &mut S) -> Option<usize>,
    ) -> Option<(usize, Vec<IVec2>)> {
        djikstra_path(
            start,
            0,
            &mut (self, extra_state),
            |&pos, _, (state, extra_state)| done(pos, state, extra_state),
            |pos, _, (state, extra_state)| {
                let x: SmallVec<[_; 4]> = get_cardinal_neighbors(*pos)
                    .iter()
                    .map(|x| {
                        if let Some(w) = get_weight(*x, state, extra_state) {
                            Some((*x, w))
                        } else {
                            None
                        }
                    })
                    .collect();
                x
            },
        )
    }
}

pub fn pretty_print_set(set: &HashSet<IVec2>, str_fn: &impl Fn(&IVec2) -> String, width: usize) {
    let max_y = set.iter().map(|pos| pos.y).max().unwrap();
    let max_x = set.iter().map(|pos| pos.x).max().unwrap();
    let min_x = set.iter().map(|pos| pos.x).min().unwrap();
    let min_y = set.iter().map(|pos| pos.y).min().unwrap();
    for y in min_y..=max_y {
        let mut row = vec![];
        for x in min_x..=max_x {
            let pos = IVec2::new(x, y);
            if let Some(val) = set.get(&pos) {
                row.push(str_fn(val).pad_to_width(width));
            } else {
                row.push(" ".to_string().pad_to_width(width));
            }
        }
        println!("{}", row.iter().join(""));
    }
}

#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dir3 {
    Left,
    Right,
    Up,
    Down,
    Forward,
    Backward,
}

impl Dir3 {
    pub fn flipped(&self) -> Dir3 {
        match self {
            Dir3::Forward => Dir3::Backward,
            Dir3::Backward => Dir3::Forward,
            Dir3::Left => Dir3::Right,
            Dir3::Right => Dir3::Left,
            Dir3::Up => Dir3::Down,
            Dir3::Down => Dir3::Up,
        }
    }
}

impl Into<Vec3> for &Dir3 {
    fn into(self) -> Vec3 {
        match self {
            Dir3::Left => Vec3::X * -1.,
            Dir3::Right => Vec3::X,
            Dir3::Up => Vec3::Y,
            Dir3::Down => Vec3::Y * -1.,
            Dir3::Forward => Vec3::Z,
            Dir3::Backward => Vec3::Z * -1.,
        }
    }
}
impl Into<Vec3> for Dir3 {
    fn into(self) -> Vec3 {
        (&self).into()
    }
}

#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dir2 {
    Left,
    Up,
    Right,
    Down,
}

impl Dir2 {
    pub fn flipped(&self) -> Dir2 {
        match self {
            Dir2::Left => Dir2::Right,
            Dir2::Right => Dir2::Left,
            Dir2::Up => Dir2::Down,
            Dir2::Down => Dir2::Up,
        }
    }

    pub fn turn_right(&self) -> Dir2 {
        let i = (ToPrimitive::to_i32(self).unwrap() + 1) % 4;
        FromPrimitive::from_i32(i).unwrap()
    }
    pub fn turn_left(&self) -> Dir2 {
        let i = (ToPrimitive::to_i32(self).unwrap() + 3) % 4;
        FromPrimitive::from_i32(i).unwrap()
    }
}

impl Into<Vec2> for &Dir2 {
    fn into(self) -> Vec2 {
        match self {
            Dir2::Left => Vec2::X * -1.,
            Dir2::Right => Vec2::X,
            Dir2::Up => Vec2::Y,
            Dir2::Down => Vec2::Y * -1.,
        }
    }
}
impl Into<Vec2> for Dir2 {
    fn into(self) -> Vec2 {
        (&self).into()
    }
}

impl Into<IVec2> for &Dir2 {
    fn into(self) -> IVec2 {
        match self {
            Dir2::Left => IVec2::X * -1,
            Dir2::Right => IVec2::X,
            Dir2::Up => IVec2::Y,
            Dir2::Down => IVec2::Y * -1,
        }
    }
}
impl Into<IVec2> for Dir2 {
    fn into(self) -> IVec2 {
        (&self).into()
    }
}

impl Into<Dir2> for &IVec2 {
    fn into(self) -> Dir2 {
        if self.x > 0 {
            return Dir2::Right;
        }
        if self.x < 0 {
            return Dir2::Left;
        }
        if self.y > 0 {
            return Dir2::Up;
        }
        return Dir2::Down;
    }
}
impl Into<Dir2> for IVec2 {
    fn into(self) -> Dir2 {
        (&self).into()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct MinWeight<T>
where
    T: Debug + PartialEq + Eq + Hash + Clone,
{
    pub weight: usize,
    pub dat: T,
}

impl<T> MinWeight<T>
where
    T: Debug + PartialEq + Eq + Hash + Clone,
{
    pub fn new(weight: usize, dat: T) -> Self {
        MinWeight { weight, dat }
    }
}

impl<T> Ord for MinWeight<T>
where
    T: Debug + PartialEq + Eq + Hash + Clone,
{
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl<T> PartialOrd for MinWeight<T>
where
    T: Debug + PartialEq + Eq + Hash + Clone,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn djikstra<T, N, S>(
    start: T,
    start_weight: usize,
    state: &mut S,
    mut done: impl FnMut(&T, usize, &mut S) -> bool,
    mut get_neighbors: impl FnMut(&T, usize, &mut S) -> N,
) -> Option<usize>
where
    T: Debug + PartialEq + Eq + Hash + Clone,
    N: IntoIterator<Item = Option<(T, usize)>>,
{
    let mut completed = HashSet::new();
    let mut heap: BinaryHeap<MinWeight<T>> = BinaryHeap::new();
    let mut distances = HashMap::new();

    distances.insert(start.clone(), start_weight);
    heap.push(MinWeight::new(start_weight, start));

    while let Some(pos) = heap.pop() {
        if !completed.contains(&pos.dat) {
            completed.insert(pos.dat.clone());
            let weight = distances[&pos.dat];

            if done(&pos.dat, pos.weight, state) {
                return Some(pos.weight);
            }

            for neigh in get_neighbors(&pos.dat, pos.weight, state) {
                if let Some((neighbor, neighbor_weight)) = neigh {
                    let new_weight = neighbor_weight + weight;
                    if new_weight < *distances.get(&neighbor).unwrap_or(&usize::MAX) {
                        distances.insert(neighbor.clone(), new_weight);
                        heap.push(MinWeight::new(new_weight, neighbor));
                    }
                }
            }
        }
    }
    None
}

#[derive(Debug, Clone)]
pub struct PathHolder<T>
where
    T: Debug + PartialEq + Eq + Hash + Clone,
{
    path: Vec<T>,
    val: T,
}

impl<T> Hash for PathHolder<T>
where
    T: Debug + PartialEq + Eq + Hash + Clone,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.val.hash(state);
    }
}

impl<T> PartialEq for PathHolder<T>
where
    T: Debug + PartialEq + Eq + Hash + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl<T> Eq for PathHolder<T>
where
    T: Debug + PartialEq + Eq + Hash + Clone,
{
    fn assert_receiver_is_total_eq(&self) {}
}

pub fn djikstra_path<T, N, S>(
    start: T,
    start_weight: usize,
    state: &mut S,
    mut done: impl FnMut(&T, usize, &mut S) -> bool,
    mut get_neighbors: impl FnMut(&T, usize, &mut S) -> N,
) -> Option<(usize, Vec<T>)>
where
    T: Debug + PartialEq + Eq + Hash + Clone,
    N: IntoIterator<Item = Option<(T, usize)>>,
{
    let mut res = None;
    djikstra(
        PathHolder {
            path: vec![],
            val: start,
        },
        start_weight,
        state,
        |val, w, state| {
            if done(&val.val, w, state) {
                res = Some((w, val.path.clone()));
                true
            } else {
                false
            }
        },
        |val, weight, state| {
            let mut new_path = val.path.clone();
            new_path.push(val.val.clone());

            get_neighbors(&val.val, weight, state)
                .into_iter()
                .map(move |n| {
                    if let Some((n, w)) = n {
                        Some((
                            (PathHolder {
                                path: new_path.clone(),
                                val: n,
                            }),
                            w,
                        ))
                    } else {
                        None
                    }
                })
        },
    );

    res
}

#[test]
pub fn base() {
    if let Ok(lines) = read_lines("./src/year2021/data/day1testinput.txt") {
        // Consumes the iterator, returns an (Optional) String
        for (line_num, line) in lines.enumerate() {
            if let Ok(contents) = line {}
        }
    }
}
