#![allow(dead_code)]

use std::{
    collections::{HashSet, VecDeque},
    fs,
    io::Read,
    sync::{Arc, Mutex},
    time::SystemTime,
    usize,
};

mod p2_shoelace;

#[tokio::main]
async fn main() {
    const INPUT: &str = "test_1.txt";
    // let plans = p1_parse_input(INPUT);
    // for p in plans.iter() {
    //     println!("{:?}", p.parse_color());
    // }
    // p2_dig(&plans).await;

    p2_shoelace::solve(INPUT);
}

async fn p2_dig(plans: &[Plan]) {
    // calculate size
    let (mut up, mut down, mut left, mut right) = (0, 0, 0, 0);
    for p in plans {
        let (dir, length) = p.parse_color();
        match dir {
            Direction::Up => up += length,
            Direction::Down => down += length,
            Direction::Left => left += length,
            Direction::Right => right += length,
        }
    }
    println!("up: [{up}], down: [{down}], left: [{left}], right: [{right}]");

    let (start_r, start_c) = ((up + down) / 2, (left + right) / 2);
    let mut map = HashSet::<(usize, usize)>::new(); // row_number -> digged collumns

    // digging
    let (mut pos_r, mut pos_c) = (start_r, start_c);
    let (mut min_r, mut max_r, mut min_c, mut max_c) = (usize::MAX, 0, usize::MAX, 0);
    for p in plans {
        let (dir, length) = p.parse_color();
        for _ in 1..=length {
            match dir {
                Direction::Up => pos_r -= 1,
                Direction::Down => pos_r += 1,
                Direction::Left => pos_c -= 1,
                Direction::Right => pos_c += 1,
            }
            map.insert((pos_r, pos_c));
            // println!("added {} {}", pos_r, pos_c);
            min_r = min_r.min(pos_r);
            max_r = max_r.max(pos_r);
            min_c = min_c.min(pos_c);
            max_c = max_c.max(pos_c);
        }
    }
    println!("digged {min_r} {max_r} {min_c} {max_c}");

    // find an inside point - by ray casting up right
    let (mut ir, mut ic) = (0, 0);
    let mut is_found = false;
    for r in min_r..=max_r {
        for c in min_c..=max_c {
            if map.contains(&(r, c)) {
                continue;
            }
            let mut ray_intersect = 0;
            let (mut ray_r, mut ray_c) = (r, c);
            loop {
                if ray_r == min_r || ray_c > (max_c - 1) {
                    break;
                }

                ray_r -= 1;
                ray_c += 1;

                // println!(
                //     "ray casting -- {} {} {} {}",
                //     ray_r,
                //     ray_c,
                //     map.contains(&(ray_r, ray_c)),
                //     p2_is_skip_corner(&map, (ray_r, ray_c)),
                // );

                if !map.contains(&(ray_r, ray_c)) || p2_is_skip_corner(&map, (ray_r, ray_c)) {
                    continue;
                }
                ray_intersect += 1;
            }

            // println!("ray casting {} {} {ray_intersect}", r, c);
            if ray_intersect % 2 == 0 {
                continue;
            }

            ir = r;
            ic = c;
            is_found = true;
            break;
        }
        if is_found {
            break;
        }
    }
    println!("found inside {} {}", ir, ic);

    // flood fill the inside
    let fill_queue: Arc<Mutex<VecDeque<(usize, usize)>>> = Arc::new(Mutex::new(VecDeque::new()));
    {
        fill_queue.lock().unwrap().push_front((ir, ic));
    }
    let fill_set = Arc::new(Mutex::new(HashSet::<(usize, usize)>::new()));

    // using concurrent power?
    const MAX_CON: usize = 32;
    let mut futures = Vec::new();
    let map = Arc::new(Mutex::new(map));

    for _ in 0..MAX_CON {
        let fill_queue = fill_queue.clone();
        let map = map.clone();
        let fill_set = fill_set.clone();

        futures.push(tokio::spawn(async move {
            loop {
                // next
                let mut next = Vec::<(usize, usize)>::new();
                {
                    {
                        let mut count = 20;
                        let mut q = fill_queue.lock().unwrap();
                        while count > 0 {
                            match q.pop_back() {
                                None => {
                                    break;
                                }
                                Some((pr, pc)) => {
                                    next.push((pr, pc));
                                    count -= 1;
                                }
                            }
                        }
                    }
                }

                // println!("next {}", next.len());

                if next.is_empty() {
                    println!("{:?} {}", SystemTime::now(), map.lock().unwrap().len());
                    continue;
                }

                {
                    let mut m = map.lock().unwrap();
                    for pos in next.iter() {
                        m.insert((pos.0, pos.1));
                        if m.len() % 2_000_000 == 0 {
                            println!("{:?} {}", SystemTime::now(), m.len());
                        }
                    }
                }

                // up
                let mut neighbors = Vec::<(usize, usize)>::new();
                for pos in next.iter() {
                    let (r, c) = (pos.0, pos.1);
                    if r > min_r {
                        neighbors.push((r - 1, c))
                    }
                    if r < max_r - 1 {
                        neighbors.push((r + 1, c));
                    }
                    if c > min_c {
                        neighbors.push((r, c - 1));
                    }
                    if c < max_c - 1 {
                        neighbors.push((r, c + 1));
                    }
                }

                {
                    let (m, s, mut q) = (
                        map.lock().unwrap(),
                        fill_set.lock().unwrap(),
                        fill_queue.lock().unwrap(),
                    );
                    for (r, c) in neighbors.iter() {
                        let (r, c) = (*r, *c);
                        if m.contains(&(r, c)) {
                            continue;
                        }
                        if s.contains(&(r, c)) {
                            continue;
                        }
                        println!("neighbors added {} {} {}", r, c, neighbors.len());
                        q.push_front((r, c));
                    }
                }
            }
        }));
    }

    for f in futures {
        f.await.unwrap();
    }
}

fn p2_is_skip_corner(map: &HashSet<(usize, usize)>, (r, c): (usize, usize)) -> bool {
    // ray casting: up right
    // consider a square -> skip top left & bottom right]

    /* Top left
    ##...
    #..
    */
    let is_top_lelf = (map.contains(&(r + 1, c))) && (map.contains(&(r, c + 1)));

    /* bottom right
    ..#
    .##
     */
    let is_bottom_right =
        (r > 0 && map.contains(&(r - 1, c))) && (c > 0 && map.contains(&(r, c - 1)));

    is_top_lelf || is_bottom_right
}

fn p1_dig(plans: &[Plan]) {
    // calculate size
    let (mut up, mut down, mut left, mut right) = (0, 0, 0, 0);
    for p in plans {
        // println!("{:?}", p);
        match p.direction {
            Direction::Up => up += p.length,
            Direction::Down => down += p.length,
            Direction::Left => left += p.length,
            Direction::Right => right += p.length,
        }
    }
    println!("up: [{up}], down: [{down}], left: [{left}], right: [{right}]");

    // create map
    let mut visualize_map = vec![vec!['.'; left + right + 1]; up + down + 1];
    let (start_r, start_c) = ((up + down) / 2, (left + right) / 2);
    let (mut pos_r, mut pos_c) = (start_r, start_c);

    for p in plans.iter() {
        println!("{:?}", p);

        for _ in 1..=p.length {
            match p.direction {
                Direction::Up => pos_r -= 1,
                Direction::Down => pos_r += 1,
                Direction::Left => pos_c -= 1,
                Direction::Right => pos_c += 1,
            }
            visualize_map[pos_r][pos_c] = '#'
        }
    }

    println!("visulize map");
    // for r in visualize_map.iter() {
    //     println!("{:?}", r);
    // }
    // println!();

    // ray casting
    let mut filled_map = visualize_map.clone();
    for r in 0..visualize_map.len() {
        for c in 0..visualize_map[0].len() {
            // println!("ray casting {r} {c}");
            if visualize_map[r][c] == '#' {
                continue;
            }

            let mut intersect_count = 0;
            let (mut ir, mut ic) = (r, c);
            loop {
                if ir == 0 || ic >= visualize_map[0].len() - 1 {
                    break;
                }
                ir -= 1;
                ic += 1;

                if visualize_map[ir][ic] == '.' {
                    continue;
                }
                if p1_is_skip_corner(&visualize_map, (ir, ic)) {
                    continue;
                }
                intersect_count += 1;
            }

            // println!("r {}, c {}, intersect {}", r, c, intersect_count);

            if intersect_count % 2 == 0 {
                continue;
            }
            filled_map[r][c] = '#';
        }
    }
    for r in filled_map.iter() {
        println!("{:?}", r);
    }
    println!();

    // count filled map
    let sum: usize = filled_map
        .iter()
        .map(|row| {
            row.iter()
                .map(|&c| if c == '#' { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum();
    println!("p1 sum: {}", sum);
}

fn p1_is_skip_corner(v_map: &[Vec<char>], (r, c): (usize, usize)) -> bool {
    // ray casting: up right
    // consider a square -> skip top left & bottom right]

    /* Top left
    ##...
    #..
    */
    let is_top_lelf = (r < v_map.len() - 1 && v_map[r + 1][c] == '#')
        && (c < v_map[0].len() - 1 && v_map[r][c + 1] == '#');

    /* bottom right
    ..#
    .##
     */
    let is_bottom_right = (r > 0 && v_map[r - 1][c] == '#') && (c > 0 && v_map[r][c - 1] == '#');

    is_top_lelf || is_bottom_right
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Plan {
    direction: Direction,
    length: usize,
    color: String,
}

impl Plan {
    fn parse_color(&self) -> (Direction, usize) {
        let dir = match self.color.chars().last().unwrap() {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _ => panic!("invalid direction char"),
        };

        let length = usize::from_str_radix(
            &self
                .color
                .chars()
                .take(self.color.len() - 1)
                .skip(1)
                .collect::<String>(),
            16,
        )
        .unwrap();

        // (self.direction, self.length)
        (dir, length)
    }
}

fn p1_parse_input(file: &str) -> Vec<Plan> {
    let mut buffer = String::new();
    fs::File::open(file)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut plans = Vec::new();

    for l in buffer.lines().filter(|l| !l.is_empty()) {
        let data: Vec<&str> = l.split(' ').collect();
        if data.len() != 3 {
            panic!("invalid data: {}", l);
        }

        let dir = match data[0] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => {
                panic!("invalid dir {}", data[0])
            }
        };

        plans.push(Plan {
            direction: dir,
            length: data[1].parse().unwrap(),
            color: data[2]
                .trim_start_matches('(')
                .trim_end_matches(')')
                .to_string(),
        })
    }

    plans
}

mod test {
    #[test]
    fn test_parse_hex() {
        let input = "70c71";

        println!("{}", usize::from_str_radix(input, 16).unwrap());
    }
}
