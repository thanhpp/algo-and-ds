use std::{collections::HashMap, fs, io::Read};

fn main() {
    let mut buffer = String::new();
    let _ = fs::File::open("input.txt")
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();
    let lines: Vec<&str> = buffer.split('\n').collect();

    let mut numbers: HashMap<&str, i64> = HashMap::new();
    numbers.insert("one", 1);
    numbers.insert("two", 2);
    numbers.insert("three", 3);
    numbers.insert("four", 4);
    numbers.insert("five", 5);
    numbers.insert("six", 6);
    numbers.insert("seven", 7);
    numbers.insert("eight", 8);
    numbers.insert("nine", 9);

    let mut sum = 0;
    for l in lines {
        let val = form_number_2(l, &numbers);
        println!("{} -> {}", l, val);
        sum += val;
    }

    println!("{}", sum)
}

fn form_number(input: &str) -> i64 {
    let (mut first, mut last): (i64, i64) = (0, 0);
    for c in input.chars() {
        if c.is_numeric() {
            let digit = c.to_digit(10).unwrap() as i64;
            if first == 0 {
                first = digit;
            }
            last = digit
        }
    }

    first * 10 + last
}

fn form_number_2(input: &str, numbers: &HashMap<&str, i64>) -> i64 {
    let (mut first, mut last): (i64, i64) = (0, 0);
    let chars: Vec<char> = input.chars().collect();

    // first
    for i in 0..chars.len() {
        if chars[i].is_numeric() {
            first = chars[i].to_digit(10).unwrap() as i64;
            break;
        }

        for offset in 2..=4 {
            if i + offset >= chars.len() {
                break;
            }

            let s = chars[i..=i + offset].iter().collect::<String>();

            // println!("{}", s);

            if let Some(n) = numbers.get(s.as_str()) {
                first = *n;
                break;
            }
        }

        if first != 0 {
            break;
        }
    }

    for i in (0..chars.len()).rev() {
        if chars[i].is_numeric() {
            last = chars[i].to_digit(10).unwrap() as i64;
            break;
        }

        for offset in 2..=4 {
            if i + offset >= chars.len() {
                break;
            }

            let s = chars[i..=i + offset].iter().collect::<String>();

            // println!("{}", s);

            if let Some(n) = numbers.get(s.as_str()) {
                last = *n;
                break;
            }
        }

        if last != 0 {
            break;
        }
    }

    first * 10 + last
}
