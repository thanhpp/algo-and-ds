use std::{fs, io::Read};

fn main() {
    const INPUT: &str = "input.txt";

    let sequences = parse_input(INPUT);
    println!("{:#?}", sequences);

    println!("part_1 {}", part_1(&sequences));

    println!("part_2 {}", part_2(&sequences));
}

fn part_2(sequences: &[Vec<isize>]) -> isize {
    let mut sum = 0;

    for s in sequences {
        sum += p2_find_sequence_value(s);
    }

    sum
}

fn part_1(sequences: &[Vec<isize>]) -> isize {
    let mut sum = 0;

    for s in sequences {
        sum += p1_find_sequence_value(s);
    }

    sum
}

fn p2_find_sequence_value(seq: &[isize]) -> isize {
    let mut seqs = Vec::<Vec<isize>>::new();
    seqs.push(seq.to_vec());

    let mut idx = 0;
    loop {
        let current_seq = &seqs[idx];
        let mut new_seq = Vec::<isize>::new();

        for window in current_seq.windows(2) {
            new_seq.push(window[1] - window[0])
        }

        let mut is_end = false;
        if new_seq.iter().sum::<isize>() == 0 {
            is_end = true
        }

        seqs.push(new_seq);

        if is_end {
            break;
        }
        idx += 1;
    }

    // println!("find_sequence_value {:?}\n {:#?}", seq, seqs);

    let mut v = 0;
    for i in (0..(seqs.len() - 1)).rev() {
        v = seqs[i].first().unwrap() - v;
    }

    v
}

fn p1_find_sequence_value(seq: &[isize]) -> isize {
    let mut seqs = Vec::<Vec<isize>>::new();
    seqs.push(seq.to_vec());

    let mut idx = 0;
    loop {
        let current_seq = &seqs[idx];
        let mut new_seq = Vec::<isize>::new();

        for window in current_seq.windows(2) {
            new_seq.push(window[1] - window[0])
        }

        let mut is_end = false;
        if new_seq.iter().sum::<isize>() == 0 {
            is_end = true
        }

        seqs.push(new_seq);

        if is_end {
            break;
        }
        idx += 1;
    }

    // println!("find_sequence_value {:?}\n {:#?}", seq, seqs);

    let mut v = 0;
    for i in (0..(seqs.len() - 1)).rev() {
        v += seqs[i].last().unwrap();
    }

    v
}

fn parse_input(file: &str) -> Vec<Vec<isize>> {
    let mut buffer = String::new();
    fs::File::open(file)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut seqs: Vec<Vec<isize>> = Vec::new();
    for l in buffer.lines() {
        seqs.push(l.trim().split(' ').map(|c| c.parse().unwrap()).collect());
    }

    seqs
}
