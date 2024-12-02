use std::{collections::HashMap, fs, io::Read};

fn main() {
    solve1();
    solve2();
}

fn solve1() {
    let (mut s1, mut s2) = read();

    s1.sort_unstable();
    s2.sort_unstable();

    let mut sum = 0;
    for (i, v) in s1.iter().enumerate() {
        sum += (v - s2[i]).abs()
    }

    println!("solve 1: {}", sum);
}

fn solve2() {
    let (s1, s2) = read();

    let mut appear = HashMap::<i64, (i64, i64)>::new();
    for v in s1.iter() {
        match appear.get_mut(v) {
            Some(v) => {
                v.0 += 1;
            }
            None => {
                appear.insert(*v, (1, 0));
            }
        }
    }

    for k in s2.iter() {
        if let Some(v) = appear.get_mut(k) {
            v.1 += 1;
        }
    }

    let mut res = 0;
    for (k, v) in appear.iter() {
        res += *k * v.0 * v.1
    }

    println!("solve2: {}", res);
}

fn read() -> (Vec<i64>, Vec<i64>) {
    let mut input = String::new();
    fs::File::open("input_1.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let lines = input.lines();
    let (mut s1, mut s2) = (vec![], vec![]);
    for l in lines.filter(|v| !v.is_empty()) {
        let data = l.split("  ").collect::<Vec<&str>>();
        s1.push(data[0].trim().parse::<i64>().unwrap());
        s2.push(data[1].trim().parse::<i64>().unwrap());
    }

    (s1, s2)
}
