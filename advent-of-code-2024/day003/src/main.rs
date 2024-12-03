use std::{fs, io::Read};

use regex::Regex;

fn main() {
    solve1();
    solve2();
}

fn solve2() {
    let data = read();

    let re = Regex::new(r"(mul)(\()([0-9]{1,3})(,)([0-9]{1,3})(\))|do\(\)|don't\(\)").unwrap();
    let mut res: i64 = 0;

    let s = data.join("");
    let reg_match = re.find_iter(&s);
    let mut flag = true;
    for org in reg_match {
        if org.as_str().eq("do()") {
            flag = true;
            continue;
        }
        if org.as_str().eq("don't()") {
            flag = false;
            continue;
        }
        if !flag {
            continue;
        }
        let nums = org
            .as_str()
            .trim_start_matches("mul(")
            .trim_end_matches(')')
            .split(',')
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        res += nums[0] * nums[1];
    }

    println!("solve2 {}", res)
}

fn solve1() {
    let data = read();

    let re = Regex::new(r"(mul)(\()([0-9]+)(,)([0-9]+)(\))").unwrap();
    let mut res: i64 = 0;

    for s in data {
        let reg_match = re.captures_iter(&s).map(|c| c.extract());
        for (_, [_, _, n1, _, n2, _]) in reg_match {
            println!("{}, {}", n1, n2);
            res += n1.parse::<i64>().unwrap() * n2.parse::<i64>().unwrap();
        }
    }

    println!("solve1: {}", res)
}

fn read() -> Vec<String> {
    let mut data = String::new();
    fs::File::open("input_1.txt")
        .unwrap()
        .read_to_string(&mut data)
        .unwrap();

    let mut res = Vec::new();

    for l in data.lines().filter(|v| !v.is_empty()) {
        res.push(l.to_string());
    }

    res
}
