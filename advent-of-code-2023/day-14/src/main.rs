use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    fs,
    hash::{Hash, Hasher},
    io::Read,
};

fn main() {
    const INPUT: &str = "input.txt";

    let map = parse_input(INPUT);
    println!("part 1: {}", part_1(&map));

    println!("part 2: {}", part_2(&map));
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

// part_2: ? idk why there are a limited state of the maps when we titled it a certain amount of cycle
fn part_2(data: &Vec<Vec<char>>) -> usize {
    const CYCLE_COUNT: usize = 500;

    // test_1: cycle 14 | hash_to_map: 38 | rotation 38; sum 64
    // input: cycle 191 | hash_to_map: 739 | rotation 739; sum 99641

    let mut hash_to_map: HashMap<u64, Vec<Vec<char>>> = HashMap::new();
    hash_to_map.insert(calculate_hash(data), data.to_vec());
    let mut rotation: HashMap<(u64, u64), u64> = HashMap::new(); // (hash, direction) -> hash (1: north, 2: west, 3: south, 4: east)

    let mut current_map: Vec<Vec<char>> = data.to_vec();
    for cycle in 1..=CYCLE_COUNT {
        let mut sum = 0;
        for (r, row) in current_map.iter().enumerate() {
            for &c in row {
                if c != 'O' {
                    continue;
                }
                sum += current_map.len() - r
            }
            // println!("r sum : {}", sum)
        }
        println!(
            "cycle {} | hash_to_map: {} | rotation {}; sum {}",
            cycle,
            hash_to_map.len(),
            rotation.len(),
            sum
        );
        let hash = calculate_hash(&current_map);
        let tilted_north = match rotation.get(&(hash, 1)) {
            Some(h) => hash_to_map.get(h).unwrap().clone(),
            None => {
                let m = tilt_north(&current_map);
                let m_hash = calculate_hash(&m);
                hash_to_map.insert(m_hash, m.clone());
                rotation.insert((hash, 1), m_hash);
                m
            }
        };

        // println!("tilted_north");
        // for r in tilted_north.iter() {
        //     println!("{:?}", r);
        // }

        let hash = calculate_hash(&tilted_north);
        let tilted_west = match rotation.get(&(hash, 2)) {
            Some(h) => hash_to_map.get(h).unwrap().clone(),
            None => {
                let m = tilt_west(&tilted_north);
                let m_hash = calculate_hash(&m);
                hash_to_map.insert(m_hash, m.clone());
                rotation.insert((hash, 2), m_hash);
                m
            }
        };

        // println!("tilted_west");
        // for r in tilted_west.iter() {
        //     println!("{:?}", r);
        // }

        let hash = calculate_hash(&tilted_west);
        let tilted_south = match rotation.get(&(hash, 3)) {
            Some(h) => hash_to_map.get(h).unwrap().clone(),
            None => {
                let m = tilt_south(&tilted_west);
                let m_hash = calculate_hash(&m);
                hash_to_map.insert(m_hash, m.clone());
                rotation.insert((hash, 3), m_hash);
                m
            }
        };

        // println!("tilted_south");
        // for r in tilted_south.iter() {
        //     println!("{:?}", r);
        // }

        let hash = calculate_hash(&tilted_south);
        let tilted_east = match rotation.get(&(hash, 4)) {
            Some(h) => hash_to_map.get(h).unwrap().clone(),
            None => {
                let m = tilt_east(&tilted_south);
                let m_hash = calculate_hash(&m);
                hash_to_map.insert(m_hash, m.clone());
                rotation.insert((hash, 4), m_hash);
                m
            }
        };

        // println!("tilted_east");
        // for r in tilted_east.iter() {
        //     println!("{:?}", r);
        // }

        current_map = tilted_east
    }

    // for t in current_map.iter() {
    //     println!("{:?}", t)
    // }

    let mut sum = 0;
    for (r, row) in current_map.iter().enumerate() {
        for &c in row {
            if c != 'O' {
                continue;
            }
            sum += current_map.len() - r
        }
        // println!("r sum : {}", sum)
    }

    sum
}

fn part_1(data: &[Vec<char>]) -> usize {
    let tilted = tilt_north(data);
    for t in tilted.iter() {
        println!("{:?}", t)
    }

    let mut sum = 0;
    for (r, row) in tilted.iter().enumerate() {
        for &c in row {
            if c != 'O' {
                continue;
            }
            sum += tilted.len() - r
        }
        // println!("r sum : {}", sum)
    }

    sum
}

fn tilt_north(data: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut map = data.to_vec();

    for r in 1..map.len() {
        for c in 0..map[r].len() {
            if map[r][c] != 'O' {
                continue;
            }
            // move north
            let mut move_to = r;
            while move_to > 0 && map[move_to - 1][c] == '.' {
                move_to -= 1;
            }
            map[r][c] = '.';
            map[move_to][c] = 'O';
        }
    }

    map
}

fn tilt_south(data: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut map = data.to_vec();

    for r in (0..map.len() - 1).rev() {
        for c in (0..map[r].len()).rev() {
            if map[r][c] != 'O' {
                continue;
            }
            // move north
            let mut move_to = r;
            while move_to < map.len() - 1 && map[move_to + 1][c] == '.' {
                move_to += 1;
            }
            map[r][c] = '.';
            map[move_to][c] = 'O';
        }
    }

    map
}

fn tilt_west(data: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut map = data.to_vec();

    for c in 1..map[0].len() {
        for r in 0..map.len() {
            if map[r][c] != 'O' {
                continue;
            }
            let mut move_to = c;
            while move_to > 0 && map[r][move_to - 1] == '.' {
                move_to -= 1
            }
            map[r][c] = '.';
            map[r][move_to] = 'O';
        }
    }

    map
}

fn tilt_east(data: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut map = data.to_vec();

    for c in (0..map[0].len() - 1).rev() {
        for r in 0..map.len() {
            if map[r][c] != 'O' {
                continue;
            }
            let mut move_to = c;
            while move_to < map[0].len() - 1 && map[r][move_to + 1] == '.' {
                move_to += 1
            }
            map[r][c] = '.';
            map[r][move_to] = 'O';
        }
    }

    map
}

fn parse_input(file: &str) -> Vec<Vec<char>> {
    let mut buffer = String::new();
    fs::File::open(file)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    buffer
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect()
}
