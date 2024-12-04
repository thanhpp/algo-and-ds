use std::{fs, io::Read};

fn main() {
    solve1();
    solve2();
}

fn read() -> Vec<Vec<char>> {
    let mut s = String::new();
    fs::File::open("input_1.txt")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();

    let mut res = vec![];

    for l in s.lines() {
        res.push(l.chars().collect());
    }

    res
}

fn solve1() {
    let data = read();
    let mut total = 0;
    let delta = [
        (0, 1),
        (1, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    for r in 0..data.len() {
        for c in 0..data[r].len() {
            for (d_x, d_y) in delta.iter() {
                if check1(&data, (r as i64, c as i64), (*d_x, *d_y)) {
                    total += 1;
                }
            }
        }
    }

    println!("solve1: {}", total)
}

fn solve2() {
    let data = read();
    let mut total = 0;

    for r in 0..data.len() {
        for c in 0..data[r].len() {
            if check2(&data, (r as i64, c as i64)) {
                total += 1;
            }
        }
    }

    println!("solve2: {}", total)
}

fn check1(data: &[Vec<char>], (r, c): (i64, i64), (delta_r, delta_c): (i64, i64)) -> bool {
    let check_chars = ['X', 'M', 'A', 'S'];
    let (max_r, max_c) = (data.len() as i64, data[0].len() as i64);

    for i in 0..4 {
        let (next_r, next_c) = (r + delta_r * i, c + delta_c * i);
        if next_r < 0 || next_c < 0 || next_r >= max_r || next_c >= max_c {
            return false;
        }
        if data[next_r as usize][next_c as usize] != check_chars[i as usize] {
            return false;
        }
    }

    // println!(
    //     "found: start: {}|{}, end {}|{}",
    //     r,
    //     c,
    //     r + delta_r * 3,
    //     c + delta_c * 3
    // );

    true
}

fn check2(data: &[Vec<char>], (r, c): (i64, i64)) -> bool {
    if data[r as usize][c as usize] != 'A' {
        return false;
    }

    let (max_r, max_c) = (data.len() as i64, data[0].len() as i64);

    let (up_left, up_right, down_left, down_right) = (
        (r - 1, c - 1),
        (r - 1, c + 1),
        (r + 1, c - 1),
        (r + 1, c + 1),
    );
    if up_left.0 < 0 || up_left.1 < 0 || up_left.0 >= max_r || up_left.1 >= max_c {
        return false;
    }
    if up_right.0 < 0 || up_right.1 < 0 || up_right.0 >= max_r || up_right.1 >= max_c {
        return false;
    }
    if down_left.0 < 0 || down_left.1 < 0 || down_left.0 >= max_r || down_left.1 >= max_c {
        return false;
    }

    if down_right.0 < 0 || down_right.1 < 0 || down_right.0 >= max_r || down_right.1 >= max_c {
        return false;
    }

    let (up_left_c, up_right_c, down_left_c, down_right_c) = (
        data[up_left.0 as usize][up_left.1 as usize],
        data[up_right.0 as usize][up_right.1 as usize],
        data[down_left.0 as usize][down_left.1 as usize],
        data[down_right.0 as usize][down_right.1 as usize],
    );

    // check first cross
    if (up_left_c, down_right_c) != ('M', 'S') && (up_left_c, down_right_c) != ('S', 'M') {
        return false;
    }
    // check second cross
    if (up_right_c, down_left_c) != ('M', 'S') && (up_right_c, down_left_c) != ('S', 'M') {
        return false;
    }

    true
}
