use std::{collections::HashSet, fs, io::Read};

fn main() {
    solve1();
}

fn read() -> Vec<Vec<char>> {
    let mut s = String::new();
    fs::File::open("input_1.txt")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();

    s.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect()
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn turn_right(heading: Direction) -> (Direction, (i64, i64)) {
    match heading {
        Direction::Up => (Direction::Right, (0, 1)),
        Direction::Right => (Direction::Down, (1, 0)),
        Direction::Down => (Direction::Left, (0, -1)),
        Direction::Left => (Direction::Up, (-1, 0)),
    }
}

fn solve1() {
    let data = read();
    let (max_r, max_c) = (data.len() as i64, data[0].len() as i64);
    let (mut current_r, mut current_c) = (0, 0);
    for (r, row) in data.iter().enumerate() {
        for (c, v) in row.iter().enumerate() {
            if v.eq(&'^') {
                current_r = r as i64;
                current_c = c as i64;
                break;
            }
        }
        if current_r != 0 {
            break;
        }
    }

    println!("start: {} {}", current_r, current_c);
    let mut walked = HashSet::<(i64, i64)>::new();
    walked.insert((current_c, current_r));

    let (mut heading, (mut delta_r, mut delta_c)) = (Direction::Up, (-1, 0));
    loop {
        let (next_r, next_c) = (current_r + delta_r, current_c + delta_c);
        if next_r < 0 || next_c < 0 || next_r >= max_r || next_c >= max_c {
            break;
        }

        // find obsticle
        if data[next_r as usize][next_c as usize] == '#' {
            (heading, (delta_r, delta_c)) = turn_right(heading);
            continue;
        }

        // continue to walk
        current_r = next_r;
        current_c = next_c;
        walked.insert((current_c, current_r));
    }

    println!("solve1: {}", walked.len());
}
