use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
    io::Read,
};

fn main() {
    let prob = Problem::from_file("test_1.txt");
    // println!("problem: {:?}", prob);
    solve1(&prob);
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub map: Vec<Vec<char>>,
    pub start: (i64, i64),
    pub end: (i64, i64),
}

impl Problem {
    pub fn from_file(p: &str) -> Self {
        let mut s = String::new();
        fs::File::open(p).unwrap().read_to_string(&mut s).unwrap();

        let map: Vec<Vec<char>> = s
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().collect())
            .collect();

        let mut start = (0, 0);
        let mut end = (0, 0);

        for (row, r) in map.iter().enumerate() {
            for (col, c) in r.iter().enumerate() {
                if c.eq(&'S') {
                    start.0 = row as i64;
                    start.1 = col as i64;
                    continue;
                };

                if c.eq(&'E') {
                    end.0 = row as i64;
                    end.1 = col as i64;
                }
            }
        }

        Problem { map, start, end }
    }
}

const MOVES: &[(i64, i64)] = &[(0, 1), (0, -1), (1, 0), (-1, 0)];

fn neighbors(
    map: &[Vec<char>],
    current: &State,
    check_cheated: &HashSet<((i64, i64), (i64, i64))>,
) -> Vec<State> {
    let mut next = vec![];
    // println!("finding neighbors: {:?}", current);
    for cost in 1..=20 {
        // println!("cost: {}", cost);
        if cost != 1 && current.cheated_location.0 .0 != -1 {
            break;
        }
        for &m in MOVES {
            let (next_r, next_c) = (current.row + m.0 * cost, current.col + m.1 * cost);
            // if current.row == 7 && current.col == 7 {
            //     println!("current: {:?}, next: {:?}", current, (next_r, next_c));
            // }

            if next_r < 0
                || next_c < 0
                || next_r as usize >= map.len()
                || next_c as usize >= map[0].len()
            {
                continue;
            }

            if map[next_r as usize][next_c as usize] == '#' {
                continue;
            }

            let cheated_location = match cost == 1 {
                true => current.cheated_location,
                false => {
                    // already cheated
                    if current.cheated_location.0 .0 != -1 {
                        current.cheated_location
                    } else {
                        let mut found_block = false;
                        for r in (current.row).min(next_r)..=(current.row).max(next_r) {
                            for c in (current.col).min(next_c)..=(current.col).max(next_c) {
                                if map[r as usize][c as usize] == '#' {
                                    found_block = true;
                                    break;
                                }
                                if found_block {
                                    break;
                                }
                            }
                        }
                        if !found_block {
                            continue;
                        }
                        ((current.row, current.col), (next_r, next_c))
                    }
                }
            };

            if check_cheated.contains(&cheated_location) {
                continue;
            }

            // let mut path = current.path.clone();
            // path.push((next_r, next_c));
            next.push(State {
                row: next_r,
                col: next_c,
                cost: current.cost + cost,
                // path,
                cheated_location,
            });
        }
    }

    next
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    pub row: i64,
    pub col: i64,
    pub cost: i64,
    // pub path: Vec<(i64, i64)>,
    pub cheated_location: ((i64, i64), (i64, i64)),
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

fn dijkstra(prob: &Problem) -> Vec<Vec<HashMap<((i64, i64), (i64, i64)), i64>>> {
    let mut cost = vec![
        vec![HashMap::<((i64, i64), (i64, i64)), i64>::new(); prob.map[0].len()];
        prob.map.len()
    ];
    let mut min_heap = BinaryHeap::<Reverse<State>>::new();
    min_heap.push(Reverse(State {
        row: prob.start.0,
        col: prob.start.1,
        cost: 0,
        // path: vec![(prob.start.0, prob.start.1)],
        cheated_location: ((-1, -1), (-1, -1)),
    }));

    let mut check_cheated = HashSet::<((i64, i64), (i64, i64))>::new();

    while let Some(Reverse(next)) = min_heap.pop() {
        // println!("visitng {:?}", next);
        if check_cheated.contains(&next.cheated_location) {
            continue;
        }

        if let Some(&prev) = cost[next.row as usize][next.col as usize].get(&next.cheated_location)
        {
            if prev < next.cost {
                continue;
            }
        }

        cost[next.row as usize][next.col as usize].insert(next.cheated_location, next.cost);

        if next.row == prob.end.0 && next.col == prob.end.1 {
            check_cheated.insert(next.cheated_location);
            // println!(
            //     "cost: {}, cheated: {:?}\n",
            //     next.cost, next.cheated_location,
            // );
            continue;
        }

        for next_state in neighbors(&prob.map, &next, &check_cheated) {
            // println!("neighbors: {:?}", ((nb_row, nb_col), nb_cost));

            min_heap.push(Reverse(next_state));
        }
    }

    cost
}

fn solve1(prob: &Problem) {
    let cost = dijkstra(prob);
    // println!("{:#?}", cost);
    let thresh_hold = cost[prob.end.0 as usize][prob.end.1 as usize]
        .get(&((-1, -1), (-1, -1)))
        .unwrap();

    let mut res = HashMap::<i64, i64>::new();
    let mut res_1 = 0;

    for (k, v) in cost[prob.end.0 as usize][prob.end.1 as usize].iter() {
        if *thresh_hold - *v >= 100 {
            res_1 += 1
        }
        if *thresh_hold - *v < 0 {
            continue;
        }
        match res.get_mut(&(*thresh_hold - *v)) {
            None => {
                res.insert(*thresh_hold - *v, 1);
            }
            Some(v) => {
                *v += 1;
            }
        }
    }
    println!("res {:?}", res);
    println!("res_1: {}", res_1);
}
