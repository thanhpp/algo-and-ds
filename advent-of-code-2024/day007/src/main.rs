use std::{
    fs,
    io::Read,
    sync::{Arc, Mutex},
};

#[tokio::main]
async fn main() {
    solve1();
    solve2().await;
}

#[derive(Clone, Debug)]
pub struct Data {
    pub target: i64,
    pub values: Vec<i64>,
}

fn read() -> Vec<Data> {
    let mut s = String::new();
    fs::File::open("input_1.txt")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();

    let mut res = vec![];

    for l in s.lines() {
        let tmp = l.split(':').collect::<Vec<&str>>();
        let target = tmp[0].trim().parse::<i64>().unwrap();
        let vals = tmp[1]
            .split(' ')
            .filter(|v| !v.is_empty())
            .map(|v| v.trim().parse::<i64>().unwrap())
            .collect();

        res.push(Data {
            target,
            values: vals,
        });
    }

    res
}

fn dfs(target: i64, vals: &[i64], idx: usize, current: i64) -> bool {
    // println!("{} | {} | {}", idx, current, target);

    if current > target {
        // println!("current > target, idx: {} |{} | {}", idx, current, target);
        return false;
    }

    if idx == vals.len() - 1 {
        if current == target {
            return true;
        }

        // println!("current != target,idx: {} | {} | {}", idx, current, target);
        return false;
    }

    dfs(target, vals, idx + 1, current + vals[idx + 1])
        || dfs(target, vals, idx + 1, current * vals[idx + 1])
}

fn solve1() {
    let data = read();
    // println!("{:?}", data);
    let mut res = 0;

    for d in data {
        if dfs(d.target, &d.values, 0, d.values[0]) {
            res += d.target;
        }
    }

    println!("solve1: {}", res)
}

fn concat(a: i64, b: i64) -> i64 {
    let mut s = a.to_string();
    s.push_str(&b.to_string());

    s.parse::<i64>().unwrap()
}

fn dfs2(target: i64, vals: &[i64], idx: usize, current: i64) -> bool {
    // println!("{} | {} | {}", idx, current, target);

    if current > target {
        // println!("current > target, idx: {} |{} | {}", idx, current, target);
        return false;
    }

    if idx == vals.len() - 1 {
        if current == target {
            return true;
        }

        // println!("current != target,idx: {} | {} | {}", idx, current, target);
        return false;
    }

    dfs2(target, vals, idx + 1, current + vals[idx + 1])
        || dfs2(target, vals, idx + 1, current * vals[idx + 1])
        || dfs2(target, vals, idx + 1, concat(current, vals[idx + 1]))
}

async fn solve2() {
    let data = read();
    let res = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for d in data {
        let d = d.clone();
        let res = res.clone();
        let f = tokio::spawn(async move {
            if dfs2(d.target, &d.values, 0, d.values[0]) {
                let mut r = res.lock().unwrap();
                *r += d.target;
            }
        });
        handles.push(f);
    }

    for f in handles {
        f.await.unwrap();
    }

    println!("solve2: {}", res.lock().unwrap())
}
