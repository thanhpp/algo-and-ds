#![allow(dead_code)]

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fs,
    io::Read,
    sync::{Arc, Mutex},
};

fn part_1(input: &Image) -> usize {
    println!("{}", input.galaxies.len());

    let sum = Arc::new(Mutex::new(0));
    let rt = tokio::runtime::Builder::new_multi_thread()
        .max_blocking_threads(10)
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        let mut futures = Vec::new();
        let data = Arc::new(input.data.clone());
        for i in 0..input.galaxies.len() {
            for j in (i + 1)..input.galaxies.len() {
                let start = input.galaxies[i].clone();
                let end = input.galaxies[j].clone();
                let async_data = data.clone();
                let f = tokio::spawn(async { find_shortest_path(async_data, start, end) });
                futures.push(f)
            }
        }
        for f in futures {
            let v = f.await.unwrap();
            {
                let mut s = sum.lock().unwrap();
                *s += v;
            }
        }
    });

    let v = sum.lock().unwrap();
    *v
}

#[derive(Clone)]
struct Position {
    row: usize,
    col: usize,
}

struct Image {
    data: Vec<Vec<char>>,
    galaxies: Vec<Position>,
}

#[derive(PartialEq)]
struct PathNode {
    row: usize,
    col: usize,
    length: usize,
    value: f64,
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.value == other.value {
            return std::cmp::Ordering::Equal;
        }
        if self.value > other.value {
            return std::cmp::Ordering::Greater;
        }
        std::cmp::Ordering::Less
    }
}

impl Eq for PathNode {}

fn find_shortest_path(map: Arc<Vec<Vec<char>>>, start: Position, end: Position) -> usize {
    let mut heap = BinaryHeap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let dst = (end.row, end.col);

    heap.push(Reverse(PathNode {
        row: start.row,
        col: start.col,
        length: 0,
        value: 0.0,
    }));

    // println!("{} {}", map.len(), map[0].len());

    while !heap.is_empty() {
        let n = heap.pop().unwrap().0;

        // println!("visiting {} {} {}", n.row, n.col, n.value);

        if n.row == end.row && n.col == end.col {
            return n.length;
        }

        if visited.contains(&(n.row, n.col)) {
            continue;
        }

        visited.insert((n.row, n.col));

        // up
        if n.row > 0 {
            let (r, c) = (n.row - 1, n.col);
            // println!("up {} {}", r, c);
            if !visited.contains(&(r, c)) {
                heap.push(Reverse(PathNode {
                    row: r,
                    col: c,
                    length: n.length + 1,
                    value: n.value + distance((r, c), dst),
                }))
            }
        }
        // down
        if n.row < map.len() - 1 {
            let (r, c) = (n.row + 1, n.col);
            // println!("down {} {}", r, c);
            if !visited.contains(&(r, c)) {
                heap.push(Reverse(PathNode {
                    row: r,
                    col: c,
                    length: n.length + 1,
                    value: n.value + distance((r, c), dst),
                }))
            }
        }
        // left
        if n.col > 0 {
            let (r, c) = (n.row, n.col - 1);
            // println!("left {} {}", r, c);
            if !visited.contains(&(r, c)) {
                heap.push(Reverse(PathNode {
                    row: r,
                    col: c,
                    length: n.length + 1,
                    value: n.value + distance((r, c), dst),
                }))
            }
        }
        // right
        if n.col < map[0].len() - 1 {
            let (r, c) = (n.row, n.col + 1);
            // println!("right {} {}", r, c);
            if !visited.contains(&(r, c)) {
                heap.push(Reverse(PathNode {
                    row: r,
                    col: c,
                    length: n.length + 1,
                    value: n.value + distance((r, c), dst),
                }))
            }
        }
    }

    0
}

fn distance(src: (usize, usize), dst: (usize, usize)) -> f64 {
    f64::sqrt(f64::powi(dst.0 as f64 - src.0 as f64, 2) + f64::powi(dst.1 as f64 - src.1 as f64, 2))
}

fn parse_input(file: &str) -> Image {
    let mut buffer = String::new();
    fs::File::open(file)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let lines: Vec<&str> = buffer.lines().collect();
    let chars_map: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let mut expanded_row: Vec<Vec<char>> = Vec::new();

    for row in chars_map.iter() {
        let mut is_empty = true;
        for c in row {
            if c.ne(&'.') {
                is_empty = false;
                break;
            }
        }
        if is_empty {
            expanded_row.push(vec!['.'; row.len()]);
        }
        expanded_row.push(row.clone());
    }

    let mut expanded_col: Vec<Vec<char>> = Vec::new();
    for col in 0..expanded_row[0].len() {
        let mut is_empty = true;
        let mut col_data = Vec::new();
        for row in expanded_row.iter() {
            if row[col] != '.' {
                is_empty = false;
            }
            col_data.push(row[col]);
        }
        if is_empty {
            expanded_col.push(vec!['.'; col_data.len()]);
        }
        expanded_col.push(col_data)
    }

    // rotate
    let mut real_map: Vec<Vec<char>> = Vec::new();
    for i in 0..expanded_col[0].len() {
        let mut real_map_row: Vec<char> = Vec::new();
        for expanded_col_row in expanded_col.iter() {
            real_map_row.push(expanded_col_row[i])
        }
        real_map.push(real_map_row);
    }

    let mut galaxies: Vec<Position> = Vec::new();
    for row in 0..real_map.len() {
        // println!("{:?}", real_map[row]);
        for col in 0..real_map[0].len() {
            if real_map[row][col] == '.' {
                continue;
            }
            galaxies.push(Position { row, col })
        }
    }

    Image {
        data: real_map,
        galaxies,
    }
}
