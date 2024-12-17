use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fs, i64,
    io::Read,
};

fn main() {
    let data = read("input_1.txt");
    solve1(&data);
}

fn read(p: &str) -> Vec<Vec<char>> {
    let mut s = String::new();
    fs::File::open(p).unwrap().read_to_string(&mut s).unwrap();

    s.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_i64(&self) -> (i64, i64) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    pub fn rotate_clockwise(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    pub fn rotate_counter_clockwise(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub row: i64,
    pub col: i64,
    pub cost: i64,
    pub path: Vec<(i64, i64)>,
    pub dir: Direction,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cost.cmp(&other.cost))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn find_next(
    map: &[Vec<char>],
    dist: &mut HashMap<((i64, i64), Direction), i64>,
    current: &Node,
) -> Vec<Node> {
    let (max_r, max_c) = (map.len() as i64, map[0].len() as i64);

    let dirs = [
        current.dir,
        current.dir.rotate_clockwise(),
        current.dir.rotate_counter_clockwise(),
    ];
    let cost = [1, 1001, 1001, 2001];
    let mut res = vec![];
    let mut next_path = current.path.clone();
    next_path.push((current.row, current.col));

    for (i, dir) in dirs.iter().enumerate() {
        let (dir_r, dir_c) = dir.to_i64();
        let (next_r, next_c) = (current.row + dir_r, current.col + dir_c);
        if next_r < 0 || next_r >= max_r || next_c < 0 || next_c >= max_c {
            continue;
        }
        let (next_r, next_c) = (next_r as usize, next_c as usize);
        if map[next_r][next_c] == '#' {
            continue;
        }
        let next_cost = current.cost + cost[i];
        if let Some(&prev_cost) = dist.get(&((next_r as i64, next_c as i64), *dir)) {
            if prev_cost < next_cost {
                continue;
            }
        }
        dist.insert(((next_r as i64, next_c as i64), *dir), next_cost);

        res.push(Node {
            row: next_r as i64,
            col: next_c as i64,
            cost: next_cost,
            path: next_path.clone(),
            dir: *dir,
        });
    }

    res
}

fn solve1(map: &[Vec<char>]) {
    let (mut start_r, mut start_c) = (-1, -1);
    let (mut end_r, mut end_c) = (-1, -1);

    for (row, r) in map.iter().enumerate() {
        for (col, v) in r.iter().enumerate() {
            if v.eq(&'S') {
                start_r = row as i64;
                start_c = col as i64;
            }
            if v.eq(&'E') {
                end_r = row as i64;
                end_c = col as i64;
            }
            if start_r != -1 && end_r != -1 {
                break;
            }
        }
        if start_r != -1 && end_r != -1 {
            break;
        }
    }
    println!("start {}, {} | end {}, {}", start_r, start_c, end_r, end_c);

    let mut min_heap = BinaryHeap::<Reverse<Node>>::new();
    let mut dist = HashMap::new();
    min_heap.push(Reverse(Node {
        row: start_r,
        col: start_c,
        cost: 0,
        path: vec![],
        dir: Direction::Right,
    }));
    dist.insert(((start_r, start_c), Direction::Right), 0);

    let mut paths = Vec::<Vec<(i64, i64)>>::new();
    let mut best_cost = i64::MAX;
    while let Some(current) = min_heap.pop() {
        if current.0.row == end_r && current.0.col == end_c {
            if current.0.cost > best_cost {
                continue;
            }
            best_cost = current.0.cost;
            paths.push(current.0.path.clone());
        }

        for next_node in find_next(map, &mut dist, &current.0) {
            min_heap.push(Reverse(next_node));
        }
    }
    println!("solve1: {}", best_cost);

    let mut set = HashSet::new();
    for path in paths {
        for point in path {
            set.insert(point);
        }
    }

    println!("solve2: {}", set.len() + 1) // add last point
}
