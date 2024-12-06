use std::{
    collections::{HashMap, HashSet},
    fs,
    io::Read,
};

fn main() {
    solve1();
    solve2();
}

fn read() -> (Vec<(i64, i64)>, Vec<Vec<i64>>) {
    let mut s = String::new();
    fs::File::open("input_1.txt")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();

    let lines = s
        .lines()
        .collect::<Vec<&str>>()
        .split(|l| l.is_empty())
        .map(|x| x.to_vec())
        .collect::<Vec<Vec<&str>>>();

    let orders: Vec<(i64, i64)> = lines[0]
        .iter()
        .map(|v| {
            let tmp: Vec<i64> = v.split('|').map(|x| x.parse::<i64>().unwrap()).collect();
            (tmp[0], tmp[1])
        })
        .collect();

    let manuals: Vec<Vec<i64>> = lines[1]
        .iter()
        .map(|v| v.split(',').map(|x| x.parse::<i64>().unwrap()).collect())
        .collect();

    (orders, manuals)
}

fn solve1() {
    let (orders, manuals) = read();
    let mut must_be_after = HashMap::<i64, HashSet<i64>>::new();
    for (before, after) in orders {
        match must_be_after.get_mut(&before) {
            None => {
                must_be_after.insert(before, HashSet::from([after]));
            }
            Some(s) => {
                s.insert(after);
            }
        }
    }

    let mut res = 0;
    for m in manuals {
        if !check_after(&must_be_after, &m) {
            continue;
        }
        println!("valid: {:?}", m);
        let mid = m[m.len() / 2];
        res += mid;
    }

    println!("solve1: {}", res);
}

fn solve2() {
    let (orders, manuals) = read();
    let mut must_be_after = HashMap::<i64, HashSet<i64>>::new();
    for (before, after) in orders {
        match must_be_after.get_mut(&before) {
            None => {
                must_be_after.insert(before, HashSet::from([after]));
            }
            Some(s) => {
                s.insert(after);
            }
        }
    }

    let mut res = 0;
    for m in manuals {
        if check_after(&must_be_after, &m) {
            continue;
        }

        for i in 0..m.len() {
            let mut used = vec![false; m.len()];
            let mut path = vec![];
            if self::dfs(&must_be_after, &m, &mut used, i, &mut path) {
                // println!("path {:?}", path);
                res += path[path.len() / 2];
                break;
            }
        }
    }

    println!("solve2: {}", res);
}

fn check_after(must_be_after: &HashMap<i64, HashSet<i64>>, manual: &[i64]) -> bool {
    for i in (1..manual.len()).rev() {
        let set = match must_be_after.get(&manual[i]) {
            None => {
                continue;
            }
            Some(v) => v,
        };
        // if any before num exist -> false
        for j in 0..i {
            if set.contains(&manual[j]) {
                return false;
            }
        }
    }

    true
}

fn dfs(
    must_be_after: &HashMap<i64, HashSet<i64>>,
    manual: &[i64],
    used: &mut [bool],
    idx: usize,
    path: &mut Vec<i64>,
) -> bool {
    path.push(manual[idx]);
    used[idx] = true;

    if path.len() == manual.len() {
        return true;
    }

    // generate next
    let mut next = Vec::<usize>::new();
    let set = match must_be_after.get(&manual[idx]) {
        None => {
            path.pop();
            used[idx] = false;
            return false;
        }
        Some(v) => v,
    };
    for (i, v) in manual.iter().enumerate() {
        if used[i] {
            continue;
        }
        if set.contains(v) {
            next.push(i);
        }
    }
    // println!("{} | next: {:?}", manual[idx], next);

    for next_idx in next {
        if self::dfs(must_be_after, manual, used, next_idx, path) {
            return true;
        }
    }

    path.pop();
    used[idx] = false;
    false
}
