use std::{fs, io::Read};

mod part_2;

fn main() {
    const INPUT: &str = "input.txt";
    println!("calculate {}", calculate(INPUT));

    let mut buffer = String::new();
    fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();
    println!("part 2 {}", part_2::solve(buffer.trim_end_matches('\n')))
}

fn calculate(file: &str) -> usize {
    let mut buffer = String::new();
    fs::File::open(file)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut sum = 0;
    let mut current = 0;
    for c in buffer.chars().filter(|&c| c != '\n') {
        if c == ',' {
            sum += current;
            current = 0;
            continue;
        }
        // println!("current - 1 {}", current);
        current += c as usize;
        // println!("current - 2 {}", current);
        current *= 17;
        // println!("current - 3 {}", current);
        current %= 256;
        // println!("current - 4 {}", current);
    }
    sum += current;

    sum
}
