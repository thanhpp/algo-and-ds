use std::{fs, io::Read};

fn main() {
    const INPUT: &str = "input.txt";
    let rows = parse_input(INPUT);

    // println!("{:#?}", parse_input(INPUT));

    println!("part 1: {}", part_1(&rows));
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

    let mut count = 0;
    p1_dfs(&mut pos.clone(), 0, arrangement, possible_place, &mut count);

    count
}

fn p1_dfs(
    pos: &mut Vec<char>,
    next_idx: usize,
    arrangement: &Vec<usize>,
    possible_place: usize,
    count: &mut usize,
) {
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
            *count += 1;
        };

        // println!("{:?}", pos);
        // println!("{:?}", current_arrangement);

        return;
    }
    if next_idx >= pos.len() {
        return;
    }

    // find next possible place
    let mut idx = 0;
    for i in next_idx..pos.len() {
        if pos[i] == '?' {
            idx = i;
            break;
        };
    }

    // replace with #
    pos[idx] = '#';
    p1_dfs(pos, idx + 1, arrangement, possible_place - 1, count);

    // replace with .
    pos[idx] = '.';
    p1_dfs(pos, idx + 1, arrangement, possible_place - 1, count);

    pos[idx] = '?';
}

fn parse_input(file: &str) -> Vec<Row> {
    let mut buffer = String::new();
    fs::File::open(file)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let lines: Vec<&str> = buffer.lines().filter(|l| !l.is_empty()).collect();

    let mut rows: Vec<Row> = Vec::new();
    for l in lines {
        let data: Vec<&str> = l.split(' ').collect();
        rows.push(Row {
            pos: data[0].chars().collect(),
            arrangement: data[1].split(',').map(|c| c.parse().unwrap()).collect(),
        })
    }

    rows
}
