use pathfinding::prelude::dijkstra;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub row: isize,
    pub col: isize,
}

impl Point {
    pub fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    pub fn neighbors(&self) -> Vec<Point> {
        vec![
            Point::new(self.row - 1, self.col),
            Point::new(self.row + 1, self.col),
            Point::new(self.row, self.col - 1),
            Point::new(self.row, self.col + 1),
        ]
    }

    pub fn manhattan_distance(&self, other: &Point) -> isize {
        (self.row - other.row).abs() + (self.col - other.col).abs()
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    pub nodes: Vec<Point>, // accessable
    pub start: Point,
    pub end: Point,
}

impl Map {
    pub fn new(data: &str) -> Map {
        let mut nodes = vec![];
        let (mut start, mut end) = (Point::new(-1, -1), Point::new(-1, -1));

        for (row, r) in data.lines().enumerate() {
            for (col, c) in r.chars().enumerate() {
                let (row, col) = (row as isize, col as isize);
                match c {
                    '.' => {
                        nodes.push(Point::new(row, col));
                    }
                    'S' => {
                        start.row = row;
                        start.col = col;
                        nodes.push(start);
                    }
                    'E' => {
                        end.row = row;
                        end.col = col;
                        nodes.push(end);
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }

        Map { nodes, start, end }
    }

    pub fn neighbors(&self, p: &Point) -> Vec<(Point, usize)> {
        p.neighbors()
            .into_iter()
            .filter(|n| self.nodes.contains(n))
            .map(|n| (n, 1))
            .collect()
    }
}

// solve2: this solution only works if every nodes is the path. Otherwise, we need to impl another find cheats function
pub fn solve2(data: &str) {
    let map = Map::new(data);
    let (path, _) = dijkstra(&map.start, |p| map.neighbors(p), |p| p.eq(&map.end)).unwrap();

    let distance_to_end = path
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &p)| (p, i))
        .collect::<Vec<_>>();

    let res = distance_to_end
        .par_iter()
        .enumerate()
        // result of the map function is the number of possible cheats per point in the path
        .map(|(i, (p1, dist_1))| {
            // check all next point, skip pass the current point
            // because it makes no sense checking backward
            distance_to_end
                .iter()
                .skip(i + 1)
                .fold(0, |mut acc, (p2, dist_2)| {
                    let diff = p1.manhattan_distance(p2);

                    // can not reach p1 to p2 in 20 steps
                    if diff > 20 {
                        return acc;
                    };

                    // check if save at least 100 steps
                    if *dist_2 - *dist_1 - (diff as usize) < 100 {
                        return acc;
                    }

                    // found a cheat
                    acc += 1;
                    acc
                })
        })
        .reduce(|| 0, |acc, x| acc + x);

    println!("solve2: {}", res);
}

// CRE: https://gist.github.com/icub3d/4aae3d73aa5154565caec0fbc0deec28
