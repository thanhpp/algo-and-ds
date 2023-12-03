use std::{
    collections::{HashMap, HashSet},
    fs,
    io::Read,
};

fn main() {
    let mut buffer = String::new();
    fs::File::open("input.txt")
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();
    let mut chars_map = parse_chars_map(&buffer);

    // println!("chars_map: {:#?}", chars_map);

    let numbers = form_numbers_map(&chars_map);

    println!(
        "sum_missing_part: {}",
        sum_missing_part(&chars_map, &numbers)
    );

    println!(
        "sum_gear_ratio: {}",
        sum_gear_ratio(&mut chars_map, &numbers)
    );
}

fn sum_gear_ratio(chars_map: &mut Vec<Vec<char>>, numbers: &[Number]) -> usize {
    let mut sum = 0;
    // position -> value
    let mut pos_to_number_idx: HashMap<(usize, usize), usize> = HashMap::new();
    for (idx, n) in numbers.iter().enumerate() {
        for c in n.index.0..=n.index.1 {
            pos_to_number_idx.insert((n.line, c), idx);
            // println!("pos_to_number: {} {} -> {}", n.line, c, n.value);
        }
    }

    let (line_bound, c_bound) = (chars_map.len(), chars_map[0].len());

    for (l, line) in chars_map.iter().enumerate() {
        for (c, symbol) in line.iter().enumerate() {
            let mut tmp = 1;
            let mut set_idx: HashSet<usize> = HashSet::new();
            if symbol.ne(&'*') {
                continue;
            }
            let pos = pos_around(l, c, line_bound, c_bound);
            // println!("l: {}, c: {}, pos: {:#?}", l, c, pos);
            for p in pos.iter() {
                match pos_to_number_idx.get(p) {
                    None => {
                        continue;
                    }
                    Some(idx) => {
                        if set_idx.contains(idx) {
                            continue;
                        }

                        set_idx.insert(*idx);
                        tmp *= numbers[*idx].value;
                    }
                }
            }
            // println!(
            //     "l: {}; c: {}, sets: {}, pos: {:#?}",
            //     l,
            //     c,
            //     set_idx.len(),
            //     pos
            // );
            if set_idx.len() == 2 {
                sum += tmp;
            }
        }
    }

    sum
}

fn pos_around(line: usize, c: usize, line_bound: usize, c_bound: usize) -> Vec<(usize, usize)> {
    let mut v = Vec::new();
    // up
    if line > 0 {
        v.push((line - 1, c));
        // top left
        if c > 0 {
            v.push((line - 1, c - 1))
        }
        // top right
        if c < c_bound - 1 {
            v.push((line - 1, c + 1))
        }
    }
    // down
    if line < line_bound - 1 {
        v.push((line + 1, c));
        // bottom left
        if c > 0 {
            v.push((line + 1, c - 1))
        }
        // bottom right
        if c < c_bound - 1 {
            v.push((line + 1, c + 1))
        }
    }
    // left
    if c > 0 {
        v.push((line, c - 1))
    }
    // right
    if c < c_bound - 1 {
        v.push((line, c + 1))
    }

    v
}

fn sum_missing_part(chars_map: &Vec<Vec<char>>, numbers: &Vec<Number>) -> usize {
    let line_length = chars_map[0].len();
    let line_count = chars_map.len();
    let mut sum = 0;

    for n in numbers {
        let mut should_add = false;

        // left
        if n.index.0 > 0 {
            should_add = is_symbol(&chars_map[n.line][n.index.0 - 1])
        }
        // right
        if !should_add && n.index.1 < line_length - 1 {
            should_add = is_symbol(&chars_map[n.line][n.index.1 + 1])
        }
        let (mut start, mut end) = (n.index.0, n.index.1);
        if n.index.0 > 0 {
            start -= 1
        }
        if n.index.1 < line_length - 1 {
            end += 1
        }
        // up - check diagonally
        if !should_add && n.line > 0 {
            for i in start..=end {
                if is_symbol(&chars_map[n.line - 1][i]) {
                    should_add = true;
                    break;
                }
            }
        }

        // down - check diagonally
        if !should_add && n.line < line_count - 1 {
            for i in start..=end {
                if is_symbol(&chars_map[n.line + 1][i]) {
                    should_add = true;
                    break;
                }
            }
        }

        if should_add {
            sum += n.value;
        }
    }

    sum
}

fn is_symbol(c: &char) -> bool {
    !c.is_numeric() && c.ne(&'.')
}

fn parse_chars_map(input: &str) -> Vec<Vec<char>> {
    let lines = input.lines();
    let mut chars_map = Vec::new();
    for l in lines {
        chars_map.push(l.chars().collect());
    }

    chars_map
}

#[derive(Debug)]
struct Number {
    value: usize,
    line: usize,
    index: (usize, usize),
}

fn form_numbers_map(input: &[Vec<char>]) -> Vec<Number> {
    let mut numbers = Vec::new();

    for (i, v) in input.iter().enumerate() {
        let mut j = 0;
        while j < v.len() {
            if !v[j].is_numeric() {
                j += 1;
                continue;
            }
            // start of a number
            let mut number_end: usize = j;
            while number_end < v.len() {
                if v[number_end].is_numeric() {
                    number_end += 1;
                    continue;
                }
                break;
            }
            let number_str = v[j..number_end].iter().collect::<String>();
            // println!("[{}] [{}] [{}]", j, number_end, number_str);

            let number = number_str.parse::<usize>().unwrap();

            numbers.push(Number {
                value: number,
                line: i,
                index: (j, number_end - 1),
            });

            j = number_end + 1;
        }
    }

    numbers
}
