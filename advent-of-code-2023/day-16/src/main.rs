use std::{
    collections::{HashSet, VecDeque},
    fs,
    io::Read,
    thread,
    time::Duration,
};

fn main() {
    const INPUT: &str = "input.txt";
    let map = part_1_input(INPUT);
    part_1_traverse(&map, ((0, 0), Direction::Right));

    part_2_traverse(&map);
}

fn part_1_input(file: &str) -> Vec<Vec<char>> {
    let mut buffer = String::new();
    fs::File::open(file)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    buffer
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect()
}

fn part_2_traverse(map: &[Vec<char>]) -> usize {
    let mut res = 0;

    // bound cols
    for r in 1..(map.len() - 1) {
        res = res
            .max(part_1_traverse(map, ((r, 0), Direction::Up)))
            .max(part_1_traverse(map, ((r, 0), Direction::Down)))
            .max(part_1_traverse(map, ((r, 0), Direction::Right)))
            .max(part_1_traverse(map, ((r, map[0].len() - 1), Direction::Up)))
            .max(part_1_traverse(
                map,
                ((r, map[0].len() - 1), Direction::Down),
            ))
            .max(part_1_traverse(
                map,
                ((r, map[0].len() - 1), Direction::Left),
            ));
    }
    // bound rows
    for c in 1..(map[0].len() - 1) {
        res = res
            .max(part_1_traverse(map, ((0, c), Direction::Left)))
            .max(part_1_traverse(map, ((0, c), Direction::Right)))
            .max(part_1_traverse(map, ((0, c), Direction::Down)))
            .max(part_1_traverse(map, ((map.len() - 1, c), Direction::Left)))
            .max(part_1_traverse(map, ((map.len() - 1, c), Direction::Right)))
            .max(part_1_traverse(map, ((map.len() - 1, c), Direction::Up)));
    }
    // 4 corners
    res = res
        // top left
        .max(part_1_traverse(map, ((0, 0), Direction::Right)))
        .max(part_1_traverse(map, ((0, 0), Direction::Down)))
        // top right;
        .max(part_1_traverse(
            map,
            ((0, map[0].len() - 1), Direction::Left),
        ))
        .max(part_1_traverse(
            map,
            ((0, map[0].len() - 1), Direction::Down),
        ))
        // bottom left;
        .max(part_1_traverse(map, ((map.len() - 1, 0), Direction::Right)))
        .max(part_1_traverse(map, ((map.len() - 1, 0), Direction::Up)))
        // bottom right
        .max(part_1_traverse(
            map,
            ((map.len() - 1, map[0].len() - 1), Direction::Left),
        ))
        .max(part_1_traverse(
            map,
            ((map.len() - 1, map[0].len() - 1), Direction::Up),
        ));

    println!("part 2 traverse {}", res);

    res
}

fn part_1_traverse(
    input: &[Vec<char>],
    ((s_row, s_col), s_direction): ((usize, usize), Direction),
) -> usize {
    let mut queue = VecDeque::<((usize, usize), Direction)>::new();
    let mut visited = HashSet::new();
    let mut beam_map = vec![vec![0; input[0].len()]; input.len()];
    let mut route_map = vec![vec!['.'; input[0].len()]; input.len()];

    queue.push_back(((s_row, s_col), s_direction));

    while !queue.is_empty() {
        let ((row, col), direction) = queue.pop_back().unwrap();
        if visited.contains(&((row, col), direction)) {
            continue;
        }
        visited.insert(((row, col), direction));
        beam_map[row][col] += 1;
        match direction {
            Direction::Up => route_map[row][col] = '^',
            Direction::Down => route_map[row][col] = 'v',
            Direction::Left => route_map[row][col] = '<',
            Direction::Right => route_map[row][col] = '>',
        }
        if let Some(directions) = part_1_next_positions(input, (row, col), direction) {
            for d in directions {
                queue.push_back(d)
            }
        }

        // for r in route_map.iter() {
        //     println!("{:?}", r)
        // }
    }

    // for r in beam_map.iter() {
    //     println!("{:?}", r)
    // }

    let mut sum = 0;
    for r in beam_map.iter() {
        for c in r.iter() {
            if *c != 0 {
                sum += 1;
            }
        }
    }
    println!("part_1_traverse sum {}", sum);

    sum
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn part_1_next_positions(
    map: &[Vec<char>],
    (row, col): (usize, usize),
    direction: Direction,
) -> Option<Vec<((usize, usize), Direction)>> {
    let current_char = map[row][col];

    // thread::sleep(Duration::from_secs(1));

    // println!(
    //     "vistiting r={} c={} direction={:?} char={}",
    //     row, col, direction, current_char
    // );

    let (r_up, r_down) = (row > 0, row < map.len() - 1);
    let (c_left, c_right) = (col > 0, col < map[0].len() - 1);

    match (current_char, direction) {
        // keeping the same direction
        ('.', Direction::Up)
        | ('\\', Direction::Left)
        | ('/', Direction::Right)
        | ('|', Direction::Up) => {
            if r_up {
                return Some(vec![((row - 1, col), Direction::Up)]);
            }
        }
        ('.', Direction::Down)
        | ('\\', Direction::Right)
        | ('/', Direction::Left)
        | ('|', Direction::Down) => {
            if r_down {
                return Some(vec![((row + 1, col), Direction::Down)]);
            }
        }
        ('.', Direction::Left)
        | ('\\', Direction::Up)
        | ('/', Direction::Down)
        | ('-', Direction::Left) => {
            if c_left {
                return Some(vec![((row, col - 1), Direction::Left)]);
            }
        }
        ('.', Direction::Right)
        | ('\\', Direction::Down)
        | ('/', Direction::Up)
        | ('-', Direction::Right) => {
            if c_right {
                return Some(vec![((row, col + 1), Direction::Right)]);
            }
        }
        ('|', Direction::Left) | ('|', Direction::Right) => {
            let mut v = Vec::new();
            if r_up {
                v.push(((row - 1, col), Direction::Up));
            }
            if r_down {
                v.push(((row + 1, col), Direction::Down));
            }
            return Some(v);
        }
        ('-', Direction::Up) | ('-', Direction::Down) => {
            let mut v = Vec::new();
            if c_left {
                v.push(((row, col - 1), Direction::Left));
            }
            if c_right {
                v.push(((row, col + 1), Direction::Right));
            }
            return Some(v);
        }

        _ => panic!(
            "invalid state, r [{}] | c [{}] | direction [{:?}]",
            row, col, direction
        ),
    }

    None
}
