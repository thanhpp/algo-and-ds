use std::{fs, io::Read};

fn main() {
    let mut buffer = String::new();
    fs::File::open("input.txt")
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();
    let chars_map = parse_chars_map(&buffer);

    // println!("chars_map: {:#?}", chars_map);

    let numbers = form_numbers_map(&chars_map);

    println!("{}", sum_missing_part(&chars_map, &numbers));
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

fn form_numbers_map(input: &Vec<Vec<char>>) -> Vec<Number> {
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
