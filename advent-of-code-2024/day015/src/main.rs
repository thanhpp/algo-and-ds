use std::{fs, io::Read, usize};

fn main() {
    let prob = read("input_1.txt");
    // println!("prob: {:?}", prob);

    solve1(&prob);
    solve2(&prob);
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_i64(&self) -> (i64, i64) {
        match *self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    pub fn from_char(c: char) -> Self {
        match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,

            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub map: Vec<Vec<char>>,
    pub directions: Vec<Direction>,
}

pub fn print_map(m: &[Vec<char>]) {
    for r in m.iter() {
        for v in r.iter() {
            print!("{}", v)
        }
        println!()
    }
}

fn read(p: &str) -> Problem {
    let mut s = String::new();
    fs::File::open(p).unwrap().read_to_string(&mut s).unwrap();

    let lines = s.lines().collect::<Vec<&str>>();
    let mut map = Vec::<Vec<char>>::new();
    let mut break_at = 0;
    for (i, v) in lines.iter().enumerate() {
        if v.is_empty() {
            break_at = i;
            break;
        };
        map.push(v.chars().collect());
    }

    let mut directions = vec![];

    for s in lines.iter().skip(break_at) {
        for c in s.chars() {
            directions.push(Direction::from_char(c));
        }
    }

    Problem { map, directions }
}

// p2: apply for left, right
fn dest_1(
    map: &[Vec<char>],
    (pos_r, pos_c): (usize, usize),
    dir: &Direction,
) -> Option<(usize, usize)> {
    let (dir_r, dir_c) = dir.to_i64();
    // println!("pos {} {} {}", pos_r, pos_c, map[pos_r][pos_c]);
    let (mut pos_r, mut pos_c) = (pos_r as i64, pos_c as i64);
    let (max_r, max_c) = (map.len() as i64, map[0].len() as i64);

    // println!("dir {} {}", dir_r, dir_c);

    loop {
        pos_r += dir_r;
        pos_c += dir_c;
        if pos_r < 0 || pos_r >= max_r || pos_c < 0 || pos_c >= max_c {
            return None;
        }
        // println!(
        //     "pos - after {} {} {}",
        //     pos_r, pos_c, map[pos_r as usize][pos_c as usize]
        // );
        if map[pos_r as usize][pos_c as usize] == '#' {
            return None;
        }
        if map[pos_r as usize][pos_c as usize] == '.' {
            return Some((pos_r as usize, pos_c as usize));
        }
    }
}

// p2: apply for left, right
fn move_1(
    map: &mut [Vec<char>],
    dir: &Direction,
    (mut dest_r, mut dest_c): (usize, usize),
    (start_r, start_c): (&mut usize, &mut usize),
) {
    let (dir_r, dir_c) = dir.to_i64();

    // start moving from dest
    while map[dest_r][dest_c] != '@' {
        let (next_r, next_c) = (
            (dest_r as i64 - dir_r) as usize,
            (dest_c as i64 - dir_c) as usize,
        );
        map[dest_r][dest_c] = map[next_r][next_c];
        dest_r = next_r;
        dest_c = next_c;
    }

    // move start pos
    map[*start_r][*start_c] = '.';
    *start_r = (*start_r as i64 + dir_r) as usize;
    *start_c = (*start_c as i64 + dir_c) as usize;
    // print_map(&map);
}

fn solve1(prob: &Problem) {
    let (mut start_r, mut start_c) = (0, 0);
    let mut map = prob.map.clone();
    for (r, row) in prob.map.iter().enumerate() {
        for (c, v) in row.iter().enumerate() {
            if v.eq(&'@') {
                start_r = r;
                start_c = c;
                break;
            }
        }
    }

    for dir in prob.directions.iter() {
        // println!("--");
        // println!("dir: {:?}", dir);
        if let Some((dest_r, dest_c)) = dest_1(&map, (start_r, start_c), dir) {
            move_1(
                &mut map,
                dir,
                (dest_r, dest_c),
                (&mut start_r, &mut start_c),
            );
        }
    }

    let mut res = 0;
    for (r, row) in map.iter().enumerate() {
        for (c, v) in row.iter().enumerate() {
            if v.eq(&'O') {
                res += 100 * r + c
            }
        }
    }

    println!("solve1: {}", res)
}

// apply for up, down
fn dest_2(
    map: &[Vec<char>],
    (pos_r, pos_c): (usize, usize),
    dir: &Direction,
) -> Option<Vec<(usize, (usize, usize))>> /*r, (left_c, right_c)*/ {
    let (mut left_c, mut right_c) = (pos_c, pos_c);
    let mut pos_r = pos_r as i64;
    let (dir_r, _) = dir.to_i64();
    let max_r = map.len() as i64;

    let mut res = vec![];

    loop {
        let next_r = pos_r + dir_r;
        if next_r < 0 || next_r >= max_r {
            return None;
        }

        // check if hit any block or meet only .
        let mut should_continue = false;
        for col in left_c..=right_c {
            if map[next_r as usize][col] == '#' {
                return None;
            }
            if map[next_r as usize][col] != '.' {
                should_continue = true;
            }
        }

        // println!(
        //     "next_r {}, left_c {}, right_c {}, should_continue {}",
        //     next_r, left_c, right_c, should_continue
        // );

        if !should_continue {
            pos_r = next_r;
            res.push((pos_r as usize, (left_c, right_c)));
            break;
        }

        // check if should expand
        if map[next_r as usize][left_c] == ']' {
            left_c -= 1
        } else {
            while map[next_r as usize][left_c] == '.' {
                left_c += 1
            }
        }
        if map[next_r as usize][right_c] == '[' {
            right_c += 1
        } else {
            while map[next_r as usize][right_c] == '.' {
                right_c -= 1
            }
        }
        pos_r = next_r;
        res.push((pos_r as usize, (left_c, right_c)));
    }

    Some(res)
}

fn move_2(map: &mut [Vec<char>], steps: &[(usize, (usize, usize))]) {
    for i in (1..steps.len()).rev() {
        let (r, _) = steps[i];
        let (next_r, (left_c, right_c)) = steps[i - 1];

        // move
        for c in left_c..=right_c {
            map[r][c] = map[next_r][c];
            // reset
            if map[next_r][c] == ']' || map[next_r][c] == '[' {
                map[next_r][c] = '.'
            }
        }
    }
}

fn solve2(prob: &Problem) {
    // convert map
    let mut converted_map = Vec::<Vec<char>>::new();
    for r in prob.map.iter() {
        let mut new_row = Vec::<char>::new();
        for v in r.iter() {
            // If the tile is #, the new map contains ## instead.
            // If the tile is O, the new map contains [] instead.
            // If the tile is ., the new map contains .. instead.
            // If the tile is @, the new map contains @. instead.
            match v {
                '#' => {
                    new_row.push('#');
                    new_row.push('#');
                }
                'O' => {
                    new_row.push('[');
                    new_row.push(']');
                }
                '.' => {
                    new_row.push('.');
                    new_row.push('.');
                }
                '@' => {
                    new_row.push('@');
                    new_row.push('.');
                }
                _ => panic!(),
            }
        }
        converted_map.push(new_row);
    }
    // print_map(&converted_map);

    let (mut start_r, mut start_c) = (0, 0);
    for (r, row) in converted_map.iter().enumerate() {
        for (c, v) in row.iter().enumerate() {
            if v.eq(&'@') {
                start_r = r;
                start_c = c;
                break;
            }
        }
    }

    for dir in prob.directions.iter() {
        // println!("----\ndir: {:?}", dir);
        match dir {
            Direction::Left | Direction::Right => {
                if let Some((dest_r, dest_c)) = dest_1(&converted_map, (start_r, start_c), dir) {
                    // println!("dest-1 {} {}", dest_r, dest_c);
                    move_1(
                        &mut converted_map,
                        dir,
                        (dest_r, dest_c),
                        (&mut start_r, &mut start_c),
                    );
                }
            }
            Direction::Up | Direction::Down => {
                if let Some(steps) = dest_2(&converted_map, (start_r, start_c), dir) {
                    // println!("dest-2 {:?}", steps);
                    move_2(&mut converted_map, &steps);
                    converted_map[start_r][start_c] = '.';
                    let (dir_r, _) = dir.to_i64();
                    start_r = (start_r as i64 + dir_r) as usize;
                    converted_map[start_r][start_c] = '@';
                }
            }
        }

        // print_map(&converted_map);
    }

    let mut res = 0;
    for (r, row) in converted_map.iter().enumerate() {
        for (c, v) in row.iter().enumerate() {
            if v.eq(&'[') {
                res += 100 * r + c
            }
        }
    }

    println!("solve2: {}", res)
}
