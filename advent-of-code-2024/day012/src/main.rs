use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    io::Read,
};

fn main() {
    let map = read("input_1.txt");
    // println!("map {:?}", map);
    let area = group_area(&map);
    // println!("area: {:?}", area);
    solve1(&map, &area);
    solve2(&map, &area);
}

fn read(p: &str) -> HashMap<(i64, i64), char> {
    let mut s = String::new();
    fs::File::open(p).unwrap().read_to_string(&mut s).unwrap();

    let raw = s
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut map = HashMap::new();
    for (i_r, row) in raw.iter().enumerate() {
        for (i_c, c) in row.iter().enumerate() {
            map.insert((i_r as i64, i_c as i64), *c);
        }
    }

    map
}

const DIRECTION: &[(i64, i64)] = &[(-1, 0), (0, -1), (1, 0), (0, 1)];

fn group_area(map: &HashMap<(i64, i64), char>) -> HashMap<(char, i64), Vec<(i64, i64)>> {
    let mut area = HashMap::new();
    let mut visited = HashSet::<(i64, i64)>::new();
    let mut i: i64 = 0;
    for ((i_r, i_c), current_name) in map.iter() {
        if visited.contains(&(*i_r, *i_c)) {
            continue;
        }
        // println!("start {} {} {}", i_r, i_c, current_name);
        i += 1;
        let mut next_move = VecDeque::new();
        next_move.push_back((*i_r, *i_c));
        visited.insert((*i_r, *i_c));
        area.insert((*current_name, i), vec![(*i_r, *i_c)]);

        // checking every next move
        while let Some((r, c)) = next_move.pop_front() {
            for (d_r, d_c) in DIRECTION {
                let (next_r, next_c) = (r + d_r, c + d_c);
                if visited.contains(&(next_r, next_c)) {
                    continue;
                }

                match map.get(&(next_r, next_c)) {
                    None => {
                        continue;
                    }
                    Some(v) => {
                        if v.ne(current_name) {
                            continue;
                        }
                        next_move.push_back((next_r, next_c));
                        visited.insert((next_r, next_c));
                        match area.get_mut(&(*current_name, i)) {
                            None => {
                                area.insert((*current_name, i), vec![(next_r, next_c)]);
                            }
                            Some(v) => {
                                v.push((next_r, next_c));
                            }
                        }
                        // println!("found {} {}", next_r, next_c);
                    }
                }
            }
        }
    }

    area
}

fn count_edge(map: &HashMap<(i64, i64), char>, (i_r, i_c): (i64, i64)) -> i64 {
    let c = match map.get(&(i_r, i_c)) {
        None => {
            return 0;
        }
        Some(c) => *c,
    };

    let mut edges = 0;
    for (d_r, d_c) in DIRECTION {
        let (next_r, next_c) = (i_r + d_r, i_c + d_c);
        match map.get(&(next_r, next_c)) {
            None => {
                edges += 1;
            }
            Some(v) => {
                if v.eq(&c) {
                    continue;
                }
                edges += 1;
            }
        }
    }

    edges
}

fn solve1(map: &HashMap<(i64, i64), char>, area: &HashMap<(char, i64), Vec<(i64, i64)>>) {
    let mut res = 0;
    for (_, points) in area {
        let mut per = 0;
        for p in points {
            per += count_edge(map, *p);
        }
        // println!("area: {:?} | {} | {}", area_name, points.len(), per);

        res += points.len() as i64 * per
    }

    println!("solve1: {}", res)
}

fn count_sides(map: &HashMap<(i64, i64), char>, name: &char, single_area: &[(i64, i64)]) -> i64 {
    let mut counted = HashSet::<((i64, i64), (i64, i64))>::new();
    let mut sides = 0;

    for &(current_r, current_c) in single_area {
        sides += check_edge(
            map,
            name,
            &mut counted,
            (current_r, current_c),
            (-1, 0),
            &[(0, -1), (0, 1)],
        );
        sides += check_edge(
            map,
            name,
            &mut counted,
            (current_r, current_c),
            (1, 0),
            &[(0, -1), (0, 1)],
        );
        sides += check_edge(
            map,
            name,
            &mut counted,
            (current_r, current_c),
            (0, 1),
            &[(-1, 0), (1, 0)],
        );
        sides += check_edge(
            map,
            name,
            &mut counted,
            (current_r, current_c),
            (0, -1),
            &[(-1, 0), (1, 0)],
        );
    }

    sides
}

fn check_edge(
    map: &HashMap<(i64, i64), char>,
    name: &char,
    counted: &mut HashSet<((i64, i64), (i64, i64))>,
    start: (i64, i64),
    direction: (i64, i64),
    possible: &[(i64, i64)],
) -> i64 {
    if counted.contains(&(start, direction)) {
        return 0;
    }
    if let Some(v) = map.get(&(start.0 + direction.0, start.1 + direction.1)) {
        if v.eq(name) {
            return 0;
        }
    }

    // so there is an edge, set counted
    for p in possible {
        let mut next = start;
        loop {
            match map.get(&next) {
                None => {
                    break;
                }
                Some(v) => {
                    if v.ne(name) {
                        break;
                    }
                    if let Some(v) = map.get(&(next.0 + direction.0, next.1 + direction.1)) {
                        if v.eq(name) {
                            break;
                        }
                    }
                    counted.insert((next, direction));
                }
            }
            next.0 += p.0;
            next.1 += p.1;
        }
    }

    1
}

fn solve2(map: &HashMap<(i64, i64), char>, area: &HashMap<(char, i64), Vec<(i64, i64)>>) {
    let mut res = 0;
    for ((name, _), points) in area {
        let sides = count_sides(map, name, points);
        // println!("area: {:?} | {} | {}", name, points.len(), sides);

        res += points.len() as i64 * sides
    }

    println!("solve2: {}", res)
}
