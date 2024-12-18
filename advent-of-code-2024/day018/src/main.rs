use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
    io::Read,
};

fn main() {
    // const CAP: (i64, i64) = (6, 6);
    // const SIMULATION_NS: i64 = 12;
    // let data = read("test_1.txt");

    const CAP: (i64, i64) = (70, 70);
    const SIMULATION_NS: i64 = 1024;
    let data = read("input_1.txt");
    println!("{:?}", data);

    solve1(&data, CAP, SIMULATION_NS);

    let mut sim_count = SIMULATION_NS;
    while let Some(()) = solve1(&data, CAP, sim_count) {
        sim_count += 1;
    }
    println!("{}", sim_count);
    println!("{:?}", data[sim_count as usize]);
}

fn read(p: &str) -> Vec<Vec<i64>> {
    let mut s = String::new();
    fs::File::open(p).unwrap().read_to_string(&mut s).unwrap();

    s.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split(',').map(|v| v.parse().unwrap()).collect())
        .collect()
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct State {
    pub x: i64,
    pub y: i64,
    pub cost: i64,
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

fn solve1(bytes: &[Vec<i64>], cap: (i64, i64), sim_count: i64) -> Option<()> {
    let mut corrupted = HashSet::<(i64, i64)>::new();
    for b in bytes.iter().take(sim_count as usize) {
        corrupted.insert((b[0], b[1]));
    }

    // print_map(cap, &corrupted);

    let (start_x, start_y) = (0, 0);
    let mut cost = HashMap::<(i64, i64), i64>::new();
    cost.insert((start_x, start_y), 0);

    let mut min_heap = BinaryHeap::<Reverse<State>>::new();
    min_heap.push(Reverse(State {
        x: start_x,
        y: start_y,
        cost: 0,
    }));

    while let Some(current_state) = min_heap.pop() {
        // println!("visiting: {:?}", current_state);
        if current_state.0.x == cap.0 && current_state.0.y == cap.1 {
            println!("solve1 {}", current_state.0.cost);
            return Some(());
        }

        for ((nx, ny), ncost) in neighbors_1(
            cap,
            &corrupted,
            &cost,
            ((current_state.0.x, current_state.0.y), current_state.0.cost),
        ) {
            cost.insert((nx, ny), ncost);
            min_heap.push(Reverse(State {
                x: nx,
                y: ny,
                cost: ncost,
            }));
        }
    }

    None
}

fn print_map(cap: (i64, i64), corrupted: &HashSet<(i64, i64)>) {
    for r in 0..=cap.0 {
        for c in 0..=cap.1 {
            if corrupted.contains(&(c, r)) {
                print!("#");
                continue;
            }
            print!(".")
        }
        println!()
    }
}

static MOVES: &[(i64, i64)] = &[(0, 1), (0, -1), (1, 0), (-1, 0)];
fn neighbors_1(
    cap: (i64, i64),
    corrupted: &HashSet<(i64, i64)>,
    cost: &HashMap<(i64, i64), i64>,
    current: ((i64, i64), i64),
) -> Vec<((i64, i64), i64)> {
    let mut res = vec![];
    for m in MOVES {
        let (next_x, next_y) = (current.0 .0 + m.0, current.0 .1 + m.1);
        if next_x < 0 || next_y < 0 || next_x > cap.0 || next_y > cap.0 {
            continue;
        }

        if corrupted.contains(&(next_x, next_y)) {
            continue;
        }

        if let Some(&cost) = cost.get(&(next_x, next_y)) {
            if cost <= (current.1 + 1) {
                continue;
            }
        }

        res.push(((next_x, next_y), current.1 + 1));
    }

    res
}
