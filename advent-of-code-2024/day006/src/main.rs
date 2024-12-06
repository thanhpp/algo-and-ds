use std::{
    collections::HashSet,
    fs,
    hash::Hash,
    io::Read,
    sync::{Arc, Mutex},
};

#[tokio::main]
async fn main() {
    let ((start_r, start_c), walked) = solve1();

    // println!("walked: {:?}", walked);

    solve2(read(), (start_r, start_c), &walked).await;
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

#[derive(Debug, PartialEq, Hash, Eq, PartialOrd, Ord, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn turn_right(heading: &Direction) -> (Direction, (i64, i64)) {
    match heading {
        Direction::Up => (Direction::Right, (0, 1)),
        Direction::Right => (Direction::Down, (1, 0)),
        Direction::Down => (Direction::Left, (0, -1)),
        Direction::Left => (Direction::Up, (-1, 0)),
    }
}

fn solve1() -> ((i64, i64), HashSet<(i64, i64)>) {
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
    let (start_r, start_c) = (current_r, current_c);

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
            (heading, (delta_r, delta_c)) = turn_right(&heading);
            continue;
        }

        // continue to walk
        current_r = next_r;
        current_c = next_c;
        walked.insert((current_r, current_c));
    }

    println!("solve1: {}", walked.len());

    ((start_r, start_c), walked)
}

async fn solve2(
    data: Vec<Vec<char>>,
    (start_r, start_c): (i64, i64),
    walked: &HashSet<(i64, i64)>,
) {
    let res = Arc::new(Mutex::new(0));
    // can_exits(data, (start_r, start_c), (6, 3));
    let mut handles = vec![];
    for (ob_r, ob_c) in walked.iter() {
        let data = data.clone();
        let (ob_r, ob_c) = (*ob_r, *ob_c);
        let res = res.clone();
        let f = tokio::spawn(async move {
            if can_exit(&data, (start_r, start_c), (ob_r, ob_c)) {
                // println!("can exit, {}-{}", ob_r, ob_c);
                return;
            }
            // println!("can't exit, {}-{}", ob_r, ob_c);
            let mut r = res.lock().unwrap();
            *r += 1;
        });
        handles.push(f);
    }
    for f in handles {
        f.await;
    }

    println!("solve2: {}", res.lock().unwrap())
}

fn can_exit(data: &[Vec<char>], (start_r, start_c): (i64, i64), (ob_r, ob_c): (i64, i64)) -> bool {
    let mut visited = HashSet::<((i64, i64), Direction)>::new();

    let (max_r, max_c) = (data.len() as i64, data[0].len() as i64);
    let (mut current_r, mut current_c) = (start_r, start_c);
    let (mut delta_r, mut delta_c, mut heading) = (-1, 0, Direction::Up);

    loop {
        if visited.contains(&((current_r, current_c), heading)) {
            return false;
        }
        visited.insert(((current_r, current_c), heading));
        // println!("visited {}|{}|{:?}", current_r, current_c, heading);

        let (next_r, next_c) = (current_r + delta_r, current_c + delta_c);
        if next_r < 0 || next_c < 0 || next_r >= max_r || next_c >= max_c {
            return true;
        }

        if data[next_r as usize][next_c as usize] == '#' || (next_r == ob_r && next_c == ob_c) {
            (heading, (delta_r, delta_c)) = turn_right(&heading);
            continue;
        }

        current_r = next_r;
        current_c = next_c;
    }
}
