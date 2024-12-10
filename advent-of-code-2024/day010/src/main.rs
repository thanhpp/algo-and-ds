use std::{collections::HashSet, fs, io::Read};

fn main() {
    let data = read("input_1.txt");
    solve1(&data);
    solve2(&data);
}

fn read(p: &str) -> Vec<Vec<char>> {
    let mut s = String::new();
    fs::File::open(p).unwrap().read_to_string(&mut s).unwrap();

    s.lines()
        .filter(|l| !l.is_empty())
        .map(|s| s.chars().collect())
        .collect()
}

fn next_pos(data: &[Vec<char>], (r, c): (i64, i64)) -> Vec<(i64, i64)> {
    let next_char = match next_char(&data[r as usize][c as usize]) {
        None => {
            return vec![];
        }
        Some(c) => c,
    };
    let mut res = vec![];
    let (max_r, max_c) = (data.len() as i64, data[0].len() as i64);
    let pos_move = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    for (delta_r, delta_c) in pos_move {
        let (next_r, next_c) = (r + delta_r, c + delta_c);
        if next_r < 0 || next_c < 0 || next_r >= max_r || next_c >= max_c {
            continue;
        }
        if data[next_r as usize][next_c as usize] != next_char {
            continue;
        }

        res.push((next_r, next_c));
    }

    res
}

fn next_char(c: &char) -> Option<char> {
    match *c {
        '0' => Some('1'),
        '1' => Some('2'),
        '2' => Some('3'),
        '3' => Some('4'),
        '4' => Some('5'),
        '5' => Some('6'),
        '6' => Some('7'),
        '7' => Some('8'),
        '8' => Some('9'),
        _ => None,
    }
}

fn dfs_solve1(data: &[Vec<char>], (r, c): (i64, i64), trail_heads: &mut HashSet<(i64, i64)>) {
    // println!("dfs_solve1: {} {}", r, c);

    if data[r as usize][c as usize] == '9' {
        if trail_heads.contains(&(r, c)) {
            return;
        }
        trail_heads.insert((r, c));
        return;
    }

    let next = next_pos(data, (r, c));
    for (next_r, next_c) in next {
        dfs_solve1(data, (next_r, next_c), trail_heads);
    }
}

fn solve1(data: &[Vec<char>]) {
    let mut res = 0;
    for (r, row) in data.iter().enumerate() {
        for (c, v) in row.iter().enumerate() {
            if *v != '0' {
                continue;
            }
            let mut trail_heads = HashSet::new();
            dfs_solve1(data, (r as i64, c as i64), &mut trail_heads);
            res += trail_heads.len();
            println!("r {}, c {}, trail_heads {:?}", r, c, trail_heads);
        }
    }

    println!("solve1: {}", res);
}

fn dfs_solve2(data: &[Vec<char>], (r, c): (i64, i64), trail_heads: &mut i64) {
    // println!("dfs_solve1: {} {}", r, c);

    if data[r as usize][c as usize] == '9' {
        *trail_heads += 1;
        return;
    }

    let next = next_pos(data, (r, c));
    for (next_r, next_c) in next {
        dfs_solve2(data, (next_r, next_c), trail_heads);
    }
}

fn solve2(data: &[Vec<char>]) {
    let mut res = 0;
    for (r, row) in data.iter().enumerate() {
        for (c, v) in row.iter().enumerate() {
            if *v != '0' {
                continue;
            }
            let mut trail_heads = 0;
            dfs_solve2(data, (r as i64, c as i64), &mut trail_heads);
            res += trail_heads;
            println!("r {}, c {}, trail_heads {:?}", r, c, trail_heads);
        }
    }

    println!("solve1: {}", res);
}
