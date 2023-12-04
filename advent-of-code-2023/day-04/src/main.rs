use std::{collections::HashSet, fs, io::Read};

fn main() {
    let mut buffer = String::new();
    fs::File::open("input.txt")
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let cards = parse_cards(&buffer);

    println!("sum_winning_points: {}", sum_winning_points(&cards));
}

fn sum_winning_points(cards: &[Card]) -> isize {
    let mut sum = 0;

    for c in cards {
        let mut win_count = 0;
        for n in c.card_numbers.iter() {
            if c.win_numbers.contains(n) {
                win_count += 1;
            }
        }
        println!("card {}: {}", c.id, win_count);
        if win_count == 0 {
            continue;
        }
        sum += 2_isize.pow(win_count - 1);
    }

    sum
}

struct Card {
    id: isize,
    card_numbers: Vec<isize>,
    win_numbers: HashSet<isize>,
}

fn parse_cards(input: &str) -> Vec<Card> {
    let mut cards = Vec::new();
    let lines = input.lines();

    for l in lines {
        let data: Vec<&str> = l.split(':').collect();
        if data.len() != 2 {
            panic!("invalid data length: {:#?}", data);
        }
        let id: isize = data[0].replace("Card ", "").trim().parse().unwrap();
        let numbers: Vec<&str> = data[1].split('|').collect();
        if numbers.len() != 2 {
            panic!("invalid numbers length: {:#?}", numbers);
        }

        let card_numbers = str_to_numbers(numbers[0]);
        let win_numbers: HashSet<isize> = HashSet::from_iter(str_to_numbers(numbers[1]));

        cards.push(Card {
            id,
            card_numbers,
            win_numbers,
        })
    }

    cards
}

fn str_to_numbers(input: &str) -> Vec<isize> {
    let nums: Vec<&str> = input.split(' ').collect();

    nums.iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().parse::<isize>().unwrap())
        .collect()
}
