use std::{fs, io::Read};

fn main() {
    solve1();
    solve2();
}

fn solve2() {
    let (safes, unsafes) = solve1();
    let mut convertable = 0;

    for e in unsafes {
        for i in 0..e.len() {
            let mut v = Vec::from(&e[0..i]);
            v.extend_from_slice(&e[i + 1..]);
            if is_safe(&v) {
                println!("convertable: {:?}", v);
                convertable += 1;
                break;
            }
        }
    }

    println!("solve2: {}", safes.len() + convertable)
}

fn solve1() -> (Vec<Vec<i64>>, Vec<Vec<i64>>) {
    let input = read();

    let mut safe_count = 0;
    let (mut safes, mut unsafes) = (vec![], vec![]);
    for e in input.iter() {
        if is_safe(e) {
            safe_count += 1;
            safes.push(e.clone());
            continue;
        }
        unsafes.push(e.clone());
    }

    println!("solve1: {}", safe_count);

    (safes, unsafes)
}

fn read() -> Vec<Vec<i64>> {
    let mut data = String::new();
    fs::File::open("input_1.txt")
        .unwrap()
        .read_to_string(&mut data)
        .unwrap();

    let mut res = vec![];

    for l in data.lines().filter(|l| !l.is_empty()) {
        res.push(l.split(' ').map(|x| x.parse().unwrap()).collect());
    }

    res
}

fn is_safe(e: &[i64]) -> bool {
    let is_increasing = (e[1] - e[0]) > 0;
    for i in 1..e.len() {
        let diff = e[i] - e[i - 1];
        if (is_increasing && diff < 0) || (!is_increasing && diff > 0) {
            return false;
        }
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
    }

    true
}
