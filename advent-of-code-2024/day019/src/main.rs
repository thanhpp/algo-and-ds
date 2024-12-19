use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    io::Read,
};

#[tokio::main]
async fn main() {
    let prob = Problem::from_file("input_1.txt");
    let solve1 = solve1(&prob);
    println!("solve1: {}", solve1);

    let solve2 = solve2(&prob);
    println!("solve2: {:?}", solve2);
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub possible_patterns: Vec<String>,
    pub towels: Vec<Vec<char>>,
}

impl Problem {
    pub fn from_file(p: &str) -> Self {
        let mut s = String::new();
        fs::File::open(p).unwrap().read_to_string(&mut s).unwrap();

        let lines = s.lines().collect::<Vec<&str>>();

        let first_line = lines.first().unwrap();

        let mut towels = vec![];
        for l in lines.iter().skip(2).filter(|l| !l.is_empty()) {
            towels.push(l.chars().collect());
        }

        Problem {
            possible_patterns: first_line
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            towels,
        }
    }
}

fn build_towel_dfs(
    patterns: &HashSet<String>,
    max_pattern_length: usize,
    towel: &[char],
    current: usize,
) -> bool {
    if current >= towel.len() {
        return true;
    }

    for i in 0..=max_pattern_length {
        let end = current + i;
        if end >= towel.len() {
            break;
        }
        let search_str = towel[current..=end].iter().collect::<String>();
        if !patterns.contains(&search_str) {
            continue;
        }

        if build_towel_dfs(patterns, max_pattern_length, towel, end + 1) {
            return true;
        }
    }

    false
}

fn solve1(prob: &Problem) -> i64 {
    let max_pattern_length = prob
        .possible_patterns
        .iter()
        .map(|s| s.len())
        .max()
        .unwrap();
    let patterns = HashSet::<String>::from_iter(prob.possible_patterns.iter().cloned());
    let mut res = 0;

    for towel in prob.towels.iter() {
        // println!("building: {:?}", towel);

        if build_towel_dfs(&patterns, max_pattern_length, towel, 0) {
            // println!("build");
            res += 1;
        }
    }

    res
}

fn count_possible_build_1(
    start_patterns: &HashSet<String>,
    max_size: usize,
    expect: &[char],
) -> usize {
    let mut dfs = VecDeque::<usize>::new();
    dfs.push_front(0);

    let mut total = 0;
    while let Some(l) = dfs.pop_back() {
        if l >= expect.len() {
            total += 1;
            continue;
        }
        for r in l..=l + max_size {
            if r >= expect.len() {
                break;
            }
            let search = expect[l..=r].iter().collect::<String>();
            // println!("checking: {}", search);
            if start_patterns.contains(&search) {
                dfs.push_front(r + 1);
            }
        }
    }

    total
}

fn dfs(memo: &mut HashMap<String, usize>, moves: &HashSet<String>, expect: &[char]) -> usize {
    if expect.is_empty() {
        return 0;
    }

    let expect_str = expect.iter().collect::<String>();
    // println!("searching: {}", expect_str);

    if let Some(&count) = memo.get(&expect_str) {
        // println!("found: {} {}", expect_str, count);
        return count;
    }

    let mut new_count = 0;
    for i in 1..expect.len() {
        if !moves.contains(&expect[..i].iter().collect::<String>()) {
            continue;
        }
        new_count += dfs(memo, moves, &expect[i..]);
        // println!(
        //     "dfs result: {}, {}, {}",
        //     expect[..i].iter().collect::<String>(),
        //     expect[i..].iter().collect::<String>(),
        //     dfs(memo, moves, &expect[i..])
        // );
    }
    if new_count != 0 {
        // println!("insert {} {}", expect_str.clone(), new_count);
        memo.insert(expect_str.clone(), new_count);
    }

    new_count
}

fn solve2(prob: &Problem) {
    let max_pattern_length = prob
        .possible_patterns
        .iter()
        .map(|s| s.len())
        .max()
        .unwrap();
    let patterns = HashSet::<String>::from_iter(prob.possible_patterns.iter().cloned());
    let mut patterns_count = HashMap::<String, usize>::new();

    for p in prob.possible_patterns.iter() {
        // println!("counting for: {}", p);
        patterns_count.insert(
            p.clone(),
            count_possible_build_1(
                &patterns,
                max_pattern_length,
                &p.chars().collect::<Vec<char>>(),
            ),
        );
    }
    // println!("count: {:?}", patterns_count);

    let mut res = 0;
    for towel in prob.towels.iter() {
        println!("towel: {:?}", towel);
        res += dfs(&mut patterns_count, &patterns, towel);
    }

    // println!("count: {:?}", patterns_count);
    println!("res: {}", res)
}
