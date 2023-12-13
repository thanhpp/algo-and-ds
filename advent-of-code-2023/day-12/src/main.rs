use std::{collections::HashMap, fs, io::Read};

use itertools::Itertools;

mod part_2;

fn main() {
    const INPUT: &str = "input.txt";
    let rows = parse_input(INPUT);

    // println!("{:#?}", parse_input(INPUT));

    // println!("part 1: {}", part_1(&rows));

    // part 2
    let mut sum = 0;
    for r in rows {
        sum += part_2::my_solve(&r.pos, &r.arrangement)
    }
    println!("sum {}", sum);
}

#[derive(Debug)]
struct Row {
    pos: Vec<char>,
    arrangement: Vec<usize>,
}

fn part_1(rows: &Vec<Row>) -> usize {
    let mut sum = 0;
    for r in rows {
        sum += p1_possible_arrangement(&r.pos, &r.arrangement);
    }

    sum
}

fn p1_possible_arrangement(pos: &Vec<char>, arrangement: &Vec<usize>) -> usize {
    let mut possible_place = 0;
    for c in pos {
        match *c {
            // '#' => fixed_spring += 1,
            '?' => possible_place += 1,
            _ => continue,
        }
    }
    let mut m = HashMap::new();
    let mut id = 0;
    for i in id..pos.len() {
        if pos[i] == '?' {
            id = i;
            break;
        };
    }
    let res = p1_dfs(
        &mut pos.clone(),
        id,
        '#',
        arrangement,
        possible_place,
        0,
        &mut m,
    ) + p1_dfs(
        &mut pos.clone(),
        id,
        '.',
        arrangement,
        possible_place,
        0,
        &mut m,
    );

    res
}

fn p1_dfs(
    pos: &mut Vec<char>,
    idx: usize,
    use_char: char,
    arrangement: &Vec<usize>,
    mut possible_place: usize,
    count: usize,
    cache: &mut HashMap<(usize, char), usize>,
) -> usize {
    if idx >= pos.len() {
        return count;
    }
    pos[idx] = use_char;
    possible_place -= 1;

    if possible_place == 0 {
        let mut current_arrangement = Vec::<usize>::new();
        let mut continuous_count = 0;
        for c in pos.iter() {
            if *c == '#' {
                continuous_count += 1;
                continue;
            }
            if continuous_count != 0 {
                current_arrangement.push(continuous_count);
                continuous_count = 0;
            }
        }
        if continuous_count != 0 {
            current_arrangement.push(continuous_count);
        }

        if current_arrangement.eq(arrangement) {
            return count + 1;
        };

        // println!("{:?} | {:?}", pos, current_arrangement);

        return count;
    }

    if let Some(cached) = cache.get(&(idx, use_char)) {
        return count + cached;
    }

    // find next possible place
    let mut id = idx + 1;
    for i in id..pos.len() {
        if pos[i] == '?' {
            id = i;
            break;
        };
    }

    // replace with #
    let v1 = p1_dfs(pos, id, '#', arrangement, possible_place, count, cache);
    // replace with .
    let v2 = p1_dfs(pos, id, '.', arrangement, possible_place, count, cache);

    println!("idx: {} | {:?}", idx, cache);
    cache.insert((id, use_char), v1 + v2);

    pos[id] = '?';

    v1 + v2
}

fn parse_input(file: &str) -> Vec<Row> {
    const FACTOR: usize = 5;

    let mut buffer = String::new();
    fs::File::open(file)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let lines: Vec<&str> = buffer.lines().filter(|l| !l.is_empty()).collect();

    let mut rows: Vec<Row> = Vec::new();
    for l in lines {
        let data: Vec<&str> = l.split(' ').collect();
        let (pos, arrangement) = (
            std::iter::once(data[0]),
            data[1]
                .split(',')
                .map(|c| c.parse().unwrap())
                .collect::<Vec<usize>>(),
        );
        let r = Row {
            pos: pos.cycle().take(FACTOR).join("?").chars().collect_vec(),
            arrangement: arrangement
                .iter()
                .cycle()
                .take(FACTOR * arrangement.len())
                .cloned()
                .collect_vec(),
        };

        println!("{:?}\n{:?}", r.pos, r.arrangement);

        rows.push(r)
    }

    rows
}
