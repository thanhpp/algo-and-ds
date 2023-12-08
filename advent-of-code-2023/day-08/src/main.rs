use std::{collections::HashMap, fs, io::Read};

fn main() {
    const INPUT: &str = "input.txt";
    let (nodes, moves) = parse_input(INPUT);
    println!("nodes {:#?}", nodes);
    println!("moves {:#?}", moves);

    // println!("p1_find_path: {}", p1_find_path(&nodes, &moves));

    println!("p2_find_path: {}", p2_find_path(&nodes, &moves));
}

pub fn p2_find_path(map: &HashMap<String, Node>, moves: &Moves) -> usize {
    let mut starts: Vec<&Node> = Vec::new();

    for k in map.keys() {
        if k.ends_with('A') {
            starts.push(map.get(k).unwrap());
        }
    }
    if p2_check_done(&starts) {
        return 0;
    }

    let mut counts = vec![0; starts.len()];
    for (i, s) in starts.iter().enumerate() {
        let mut current_node = map.get(&s.name).unwrap();
        let mut idx = 0;
        let mut count = 0_usize;
        loop {
            if idx >= moves.directions.len() {
                idx = 0; // reset
            }
            if moves.directions[idx] == Direction::Right {
                current_node = map.get(&current_node.right).unwrap();
            } else {
                current_node = map.get(&current_node.left).unwrap();
            }

            idx += 1;
            count += 1;
            if current_node.name.ends_with('Z') {
                counts[i] = count;
                break;
            }
        }
    }

    println!("counts: {:?}", counts);

    lcm_vec(&counts)
}

pub fn p2_check_done(nodes: &Vec<&Node>) -> bool {
    for n in nodes {
        if !n.name.ends_with('Z') {
            return false;
        }
    }

    true
}

pub fn p1_find_path(map: &HashMap<String, Node>, moves: &Moves) -> usize {
    let start = "AAA";
    let end = "ZZZ";

    let mut current_node = map.get(start).unwrap();
    let mut idx = 0;
    let mut count = 0;
    loop {
        if idx >= moves.directions.len() {
            idx = 0; // reset
        }
        if moves.directions[idx] == Direction::Right {
            current_node = map.get(&current_node.right).unwrap();
        } else {
            current_node = map.get(&current_node.left).unwrap();
        }

        idx += 1;
        count += 1;
        if current_node.name.eq(end) {
            return count;
        }
    }
}

#[derive(Default, Debug)]
pub struct Node {
    name: String,
    left: String,
    right: String,
}

#[derive(Debug)]
pub struct Moves {
    directions: Vec<Direction>,
}

#[derive(Debug, PartialEq)]
enum Direction {
    Right,
    Left,
}

fn parse_input(file: &str) -> (HashMap<String, Node>, Moves) {
    let mut buffer = String::new();
    fs::File::open(file)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let lines: Vec<&str> = buffer.lines().collect();
    // moves
    let moves = lines[0];
    let mut input_move = Moves {
        directions: Vec::new(),
    };
    for c in moves.chars() {
        if c == 'R' {
            input_move.directions.push(Direction::Right)
        } else {
            input_move.directions.push(Direction::Left)
        }
    }

    let mut nodes = HashMap::<String, Node>::new();
    for l in lines.iter().skip(1) {
        if l.is_empty() {
            continue;
        }

        let mut n = Node::default();
        let data_1: Vec<&str> = l.split('=').collect();
        if data_1.len() != 2 {
            panic!("{:?}", data_1)
        }
        n.name = data_1[0].trim().to_string();

        let data_2: Vec<&str> = data_1[1].split(',').collect();
        n.left = data_2[0].replace('(', "").trim().to_string();
        n.right = data_2[1].replace(')', "").trim().to_string();

        nodes.insert(n.name.clone(), n);
    }

    (nodes, input_move)
}

fn lcm_vec(v: &[usize]) -> usize {
    let mut u_1 = v[0];

    for v in v.iter().skip(1) {
        u_1 = lcm(u_1, *v)
    }

    u_1
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
