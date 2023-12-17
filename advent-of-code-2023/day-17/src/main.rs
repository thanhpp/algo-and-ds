use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs,
    io::Read,
};

fn main() {
    const INPUT: &str = "input.txt";
    let map = part_1_parse_input(INPUT);
    part_1_traverse(&map, false);
    part_1_traverse(&map, true);
}

fn part_1_traverse(map: &[Vec<usize>], is_ultra: bool) {
    //  dijkstra
    let (s_row, s_col) = (0, 0);
    let (d_row, d_col) = (map.len() - 1, map[0].len() - 1);

    let mut heap = BinaryHeap::<Reverse<Node>>::new();
    let mut distances = HashMap::<(usize, usize, Direction, usize), usize>::new();
    let mut trace_map = vec![vec![(0, 0); map[0].len()]; map.len()];

    heap.push(Reverse(Node {
        row: s_row,
        col: s_col,
        distance: 0,
        direction: Direction::Start,
        direction_count: 1,
    }));

    while !heap.is_empty() {
        let node = heap.pop().unwrap().0; // node.distance = minimum distance from start to node

        if node.row == d_row && node.col == d_col {
            if is_ultra && node.direction_count < 4 {
                continue;
            }
            println!(
                "{} {} {} | {:?} {}",
                node.row, node.col, node.distance, node.direction, node.direction_count
            );

            break;
        }

        for (n_row, n_col, n_dir, n_count) in neighbors(
            map,
            (node.row, node.col),
            (node.direction, node.direction_count),
            is_ultra,
        ) {
            let n_dist = node.distance + map[n_row][n_col];

            let &dist = distances
                .get(&(n_row, n_col, n_dir, n_count))
                .unwrap_or(&usize::MAX);

            if dist > n_dist {
                distances.insert((n_row, n_col, n_dir, n_count), n_dist);
                trace_map[n_row][n_col] = (node.row, node.col);
                heap.push(Reverse(Node {
                    row: n_row,
                    col: n_col,
                    distance: n_dist,
                    direction: n_dir,
                    direction_count: n_count,
                }));
                // println!("heap added {} {} {}", n_row, n_col, n_dist);
            }
        }
    }

    println!();
}

fn neighbors(
    map: &[Vec<usize>],
    (row, col): (usize, usize),
    (direction, count): (Direction, usize),
    is_ultra: bool,
) -> Vec<(usize, usize, Direction, usize)> {
    let (mut min_step, mut max_step) = (1, 3);
    if is_ultra {
        (min_step, max_step) = (4, 10)
    }

    let mut v = Vec::new();
    let (r_up, r_down, c_left, c_right) = (
        row > 0,
        row < map.len() - 1,
        col > 0,
        col < map[0].len() - 1,
    );
    // continue
    match direction {
        Direction::Start => {
            if c_right {
                v.push((row, col + 1, Direction::Right, count + 1))
            }
            if r_down {
                v.push((row + 1, col, Direction::Down, count + 1))
            }
            return v;
        }
        Direction::Up => {
            if count < max_step && r_up {
                v.push((row - 1, col, Direction::Up, count + 1))
            }
            if c_left && count >= min_step {
                v.push((row, col - 1, Direction::Left, 1))
            }
            if c_right && count >= min_step {
                v.push((row, col + 1, Direction::Right, 1))
            }
        }
        Direction::Down => {
            if count < max_step && r_down {
                v.push((row + 1, col, Direction::Down, count + 1))
            }
            if c_left && count >= min_step {
                v.push((row, col - 1, Direction::Left, 1))
            }
            if c_right && count >= min_step {
                v.push((row, col + 1, Direction::Right, 1))
            }
        }
        Direction::Left => {
            if count < max_step && c_left {
                v.push((row, col - 1, Direction::Left, count + 1))
            }
            if r_up && count >= min_step {
                v.push((row - 1, col, Direction::Up, 1))
            }
            if r_down && count >= min_step {
                v.push((row + 1, col, Direction::Down, 1))
            }
        }
        Direction::Right => {
            if count < max_step && c_right {
                v.push((row, col + 1, Direction::Right, count + 1))
            }
            if r_up && count >= min_step {
                v.push((row - 1, col, Direction::Up, 1))
            }
            if r_down && count >= min_step {
                v.push((row + 1, col, Direction::Down, 1))
            }
        }
    }

    // println!(
    //     "neighbors {} {} {:?} {} -> {} {} {} {}\n --> {:?}",
    //     row, col, direction, count, r_up, r_down, c_left, c_right, &v
    // );

    v
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
enum Direction {
    Start,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Eq)]
struct Node {
    row: usize,
    col: usize,
    distance: usize, // from start
    direction: Direction,
    direction_count: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

fn part_1_parse_input(file: &str) -> Vec<Vec<usize>> {
    let mut buffer = String::new();
    fs::File::open(file)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    buffer
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect()
}
