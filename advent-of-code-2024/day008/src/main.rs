use std::{
    collections::{HashMap, HashSet},
    fs,
    io::Read,
};

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
        .map(|l| l.chars().collect())
        .collect()
}

fn solve1(data: &[Vec<char>]) {
    let mut antennas = HashMap::<char, Vec<(i64, i64)>>::new();
    for (r, row) in data.iter().enumerate() {
        for (c, v) in row.iter().enumerate() {
            if *v == '.' {
                continue;
            }
            match antennas.get_mut(v) {
                None => {
                    antennas.insert(*v, vec![(r as i64, c as i64)]);
                }
                Some(pos) => {
                    pos.push((r as i64, c as i64));
                }
            }
        }
    }
    let (max_r, max_c) = (data.len() as i64, data[0].len() as i64);

    let mut new_map = Vec::from(data);
    let mut unique_pos: HashSet<(i64, i64)> = HashSet::<(i64, i64)>::new();
    // calculate antinodes
    for (_, positions) in antennas.iter() {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let (i_r, i_c) = positions[i];
                let (j_r, j_c) = positions[j];
                let (a_r_1, a_c_1) = (i_r + (j_r - i_r) * 2, i_c + (j_c - i_c) * 2);
                if a_r_1 >= 0 && a_r_1 < max_r && a_c_1 >= 0 && a_c_1 < max_c {
                    // println!("found antinodes: {} | {}", a_r_1, a_c_1);
                    unique_pos.insert((a_r_1, a_c_1));
                    new_map[a_r_1 as usize][a_c_1 as usize] = '#';
                }
                let (a_r_2, a_c_2) = (j_r + (i_r - j_r) * 2, j_c + (i_c - j_c) * 2);
                if a_r_2 >= 0 && a_r_2 < max_r && a_c_2 >= 0 && a_c_2 < max_c {
                    // println!("found antinodes: {} | {}", a_r_2, a_c_2);
                    unique_pos.insert((a_r_2, a_c_2));
                    new_map[a_r_2 as usize][a_c_2 as usize] = '#';
                }
            }
        }
    }

    println!("solve1: {}", unique_pos.len());
}

fn solve2(data: &[Vec<char>]) {
    let mut antennas = HashMap::<char, Vec<(i64, i64)>>::new();
    for (r, row) in data.iter().enumerate() {
        for (c, v) in row.iter().enumerate() {
            if *v == '.' {
                continue;
            }
            match antennas.get_mut(v) {
                None => {
                    antennas.insert(*v, vec![(r as i64, c as i64)]);
                }
                Some(pos) => {
                    pos.push((r as i64, c as i64));
                }
            }
        }
    }
    let (max_r, max_c) = (data.len() as i64, data[0].len() as i64);

    let mut new_map = Vec::from(data);
    let mut unique_pos: HashSet<(i64, i64)> = HashSet::<(i64, i64)>::new();
    // calculate antinodes
    for (_, positions) in antennas.iter() {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let (i_r, i_c) = positions[i];
                let (j_r, j_c) = positions[j];
                let mut multiplier = 1;
                loop {
                    let (a_r_1, a_c_1) = (
                        i_r + (j_r - i_r) * multiplier,
                        i_c + (j_c - i_c) * multiplier,
                    );
                    if a_r_1 >= 0 && a_r_1 < max_r && a_c_1 >= 0 && a_c_1 < max_c {
                        // println!("found antinodes: {} | {}", a_r_1, a_c_1);
                        unique_pos.insert((a_r_1, a_c_1));
                        if new_map[a_r_1 as usize][a_c_1 as usize] == '.' {
                            new_map[a_r_1 as usize][a_c_1 as usize] = '#';
                        }
                        multiplier += 1;
                        continue;
                    };
                    break;
                }
                let mut multiplier = 1;
                loop {
                    let (a_r_2, a_c_2) = (
                        j_r + (i_r - j_r) * multiplier,
                        j_c + (i_c - j_c) * multiplier,
                    );
                    if a_r_2 >= 0 && a_r_2 < max_r && a_c_2 >= 0 && a_c_2 < max_c {
                        // println!("found antinodes: {} | {}", a_r_2, a_c_2);
                        unique_pos.insert((a_r_2, a_c_2));
                        if new_map[a_r_2 as usize][a_c_2 as usize] == '.' {
                            new_map[a_r_2 as usize][a_c_2 as usize] = '#';
                        }
                        multiplier += 1;
                        continue;
                    };
                    break;
                }
            }
        }
    }

    println!("solve2: {}", unique_pos.len());
    for r in new_map.iter() {
        println!("{:?}", r);
    }
}
