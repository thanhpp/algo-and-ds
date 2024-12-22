use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fmt::{Debug, Write},
    fs, i64,
    io::Read,
};

fn main() {
    let data = read("input_1.txt");
    println!("data: {:?}", data);

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

const MAP1: &[&[char]] = &[
    &['7', '8', '9'],
    &['4', '5', '6'],
    &['1', '2', '3'],
    &['X', '0', 'A'],
];

const MAP2: &[&[char]] = &[&['X', '^', 'A'], &['<', 'v', '>']];

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Action {
    Up,
    Right,
    Down,
    Left,
    Push,
}

impl Action {
    pub fn to_move(&self) -> (i64, i64) {
        match self {
            Action::Up => (-1, 0),
            Action::Right => (0, 1),
            Action::Down => (1, 0),
            Action::Left => (0, -1),
            Action::Push => (0, 0),
        }
    }

    pub fn iterator() -> std::slice::Iter<'static, Action> {
        static DIRECTIONS: [Action; 4] = [Action::Up, Action::Right, Action::Down, Action::Left];
        DIRECTIONS.iter()
    }

    pub fn to_char(&self) -> char {
        match self {
            Action::Up => '^',
            Action::Right => '>',
            Action::Down => 'v',
            Action::Left => '<',
            Action::Push => 'A',
        }
    }
}

impl Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.to_char())
    }
}

fn search_map(map: &[&[char]], search_for: &char) -> (i64, i64) {
    for (row, r) in map.iter().enumerate() {
        for (col, c) in r.iter().enumerate() {
            if c.eq(search_for) {
                return (row as i64, col as i64);
            }
        }
    }

    (-1, -1)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    pub row: i64,
    pub col: i64,
    pub cost: i64,
    pub directions: Vec<Action>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn travel_map(map: &[&[char]], start_char: char, end: char) -> Vec<Vec<Action>> {
    let start = search_map(map, &start_char);
    let end = search_map(map, &end);
    if start.0 == -1 || end.0 == -1 {
        return vec![];
    }

    let mut res = vec![];
    let mut min_step = i64::MAX;
    let mut cost = vec![vec![i64::MAX; map[0].len()]; map.len()];
    let mut min_heap = BinaryHeap::<Reverse<State>>::new();
    min_heap.push(Reverse(State {
        row: start.0,
        col: start.1,
        cost: 0,
        directions: vec![],
    }));

    while let Some(Reverse(current)) = min_heap.pop() {
        if current.row == end.0 && current.col == end.1 {
            if current.cost > min_step {
                continue;
            }
            if current.cost < min_step {
                min_step = current.cost
            }
            res.push(current.directions.clone());
        }

        if cost[current.row as usize][current.col as usize] < current.cost {
            continue;
        }

        cost[current.row as usize][current.col as usize] = current.cost;

        for dir in Action::iterator() {
            let next = dir.to_move();
            let (next_row, next_col) = (current.row + next.0, current.col + next.1);
            if next_row < 0
                || next_col < 0
                || next_row >= map.len() as i64
                || next_col >= map[0].len() as i64
            {
                continue;
            }

            let next_char = map[next_row as usize][next_col as usize];
            if next_char == 'X' {
                continue;
            }

            let mut dirs = current.directions.clone();
            dirs.push(*dir);

            min_heap.push(Reverse(State {
                row: next_row,
                col: next_col,
                cost: current.cost + 1,
                directions: dirs,
            }));
        }
    }

    res
}

fn all_shortest_path(map: &[&[char]], sequence: &[char]) -> Vec<Vec<Action>> {
    // println!("finding: {:?}", sequence);
    let mut actions = Vec::<Vec<Action>>::new();

    actions.append(&mut travel_map(map, 'A', sequence[0]));
    for dir in actions.iter_mut() {
        dir.push(Action::Push);
    }
    // println!("first map - d1 (1): {:?}", actions);

    for i in 0..sequence.len() - 1 {
        let mut tmp_dirs = vec![];
        let next_dirs = travel_map(map, sequence[i], sequence[i + 1]);
        for dir_1 in actions.iter() {
            for next_dir in next_dirs.iter() {
                let mut tmp = vec![];
                tmp.append(&mut dir_1.clone());
                tmp.append(&mut next_dir.clone());
                tmp.push(Action::Push);
                tmp_dirs.push(tmp);
            }
        }
        actions = tmp_dirs;
        // println!("first map - d1 (2): {:?}", actions);
    }
    // println!("first map - d1 (3): {:?}", actions);
    filter_shortest(actions)
}

fn filter_shortest(actions: Vec<Vec<Action>>) -> Vec<Vec<Action>> {
    let min_step = actions.iter().map(|s| s.len()).min().unwrap();

    actions
        .iter()
        .filter(|s| s.len() == min_step)
        .cloned()
        .collect()
}

fn solve1(data: &[Vec<char>]) {
    let mut solve1 = 0;
    for d in data {
        // println!("solving: {:?}", d);

        let shortest_1 = all_shortest_path(MAP1, d);
        // println!("shortest_1: {:?}", shortest_1);

        let mut shortest_2 = vec![];
        for s_1 in shortest_1.iter() {
            shortest_2.append(&mut all_shortest_path(
                MAP2,
                &s_1.iter().map(|a| a.to_char()).collect::<Vec<char>>(),
            ));
        }
        shortest_2 = filter_shortest(shortest_2);
        // println!("shortest_2: {:?}", shortest_2);

        let mut shortest_3 = vec![];
        for s_2 in shortest_2.iter() {
            shortest_3.append(&mut all_shortest_path(
                MAP2,
                &s_2.iter().map(|a| a.to_char()).collect::<Vec<char>>(),
            ));
        }
        shortest_3 = filter_shortest(shortest_3);

        println!("shortest_3: {:?}", shortest_3[0].len());

        solve1 += String::from_iter(d.iter())
            .trim_end_matches('A')
            .parse::<i64>()
            .unwrap()
            * shortest_3[0].len() as i64
    }

    println!("solve1 {}", solve1)
}
