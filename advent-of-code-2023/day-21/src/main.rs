use std::{
    collections::{HashSet, VecDeque},
    fs,
    io::Read,
};

fn main() {
    const INPUT: &str = "test_1.txt";
    let map = p1_parse_map(INPUT);
    p1_solve(&map);
}

fn p1_solve(m: &Map) {
    const MAX_STEP: usize = 64;

    let mut queue = VecDeque::<(usize, usize)>::new();

    // add start point
    queue.push_front(m.start);

    // traverse - bfs
    for step in 0..MAX_STEP {
        let queue_length = queue.len();
        let mut visited = HashSet::<(usize, usize)>::new();

        for _ in 0..queue_length {
            let pos = queue.pop_back().unwrap();
            for n in m.neighbors(pos) {
                if visited.contains(&n) {
                    continue;
                }
                visited.insert(n);
                queue.push_front(n);
            }
        }
        println!("step {} {} {}", step, queue.len(), visited.len())
    }

    println!("p1 {}", queue.len() + 1)
}

struct Map {
    m: Vec<Vec<char>>,
    start: (usize, usize),
}

impl Map {
    fn neighbors(&self, (r, c): (usize, usize)) -> Vec<(usize, usize)> {
        let mut v = vec![];
        if r > 0 && self.m[r - 1][c] == '.' {
            v.push((r - 1, c))
        }
        if r < self.m.len() - 1 && self.m[r + 1][c] == '.' {
            v.push((r + 1, c))
        }
        if c > 0 && self.m[r][c - 1] == '.' {
            v.push((r, c - 1))
        }
        if c < self.m[0].len() - 1 && self.m[r][c + 1] == '.' {
            v.push((r, c + 1))
        }

        v
    }
}

fn p1_parse_map(file: &str) -> Map {
    let mut buffer = String::new();
    fs::File::open(file)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let d: Vec<Vec<char>> = buffer
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let (mut start_r, mut start_c) = (0, 0);
    for (ir, r) in d.iter().enumerate() {
        for (ic, c) in r.iter().enumerate() {
            if *c == 'S' {
                start_r = ir;
                start_c = ic;
                break;
            }
        }
    }

    Map {
        m: d,
        start: (start_r, start_c),
    }
}
