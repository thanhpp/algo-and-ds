use std::{collections::HashMap, fs, io::Read};

fn main() {
    const INPUT_FILE: &str = "input.txt";
    let hands = parse_input(INPUT_FILE);
    println!("{:#?}", hands);

    println!("part_1: {:#?}", part_1(&hands));

    println!("part_2: {:#?}", part_2(&hands));
}

fn part_1(hands: &[Hand]) -> usize {
    let mut hands = hands.clone().to_vec();
    hands.sort_by(|a, b| {
        match a
            .type_of_hand(false)
            .partial_cmp(&b.type_of_hand(false))
            .unwrap()
        {
            std::cmp::Ordering::Equal => {
                for i in 0..5 {
                    let ord = p1_card_to_value(&a.cards[i])
                        .partial_cmp(&p1_card_to_value(&b.cards[i]))
                        .unwrap();
                    if ord.is_eq() {
                        continue;
                    }
                    return ord;
                }
                std::cmp::Ordering::Equal
            }
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        }
    });

    let mut total_winning = 0;
    for (i, h) in hands.iter().enumerate() {
        total_winning += (i + 1) * h.bet
    }

    total_winning
}

fn part_2(hands: &[Hand]) -> usize {
    let mut hands = hands.clone().to_vec();
    hands.sort_by(|a, b| {
        match a
            .type_of_hand(true)
            .partial_cmp(&b.type_of_hand(true))
            .unwrap()
        {
            std::cmp::Ordering::Equal => {
                for i in 0..5 {
                    let ord = p2_card_to_value(&a.cards[i])
                        .partial_cmp(&p2_card_to_value(&b.cards[i]))
                        .unwrap();
                    if ord.is_eq() {
                        continue;
                    }
                    return ord;
                }
                std::cmp::Ordering::Equal
            }
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        }
    });

    for h in hands.iter() {
        println!(
            "cards {:?}, bet {}, type {}",
            h.cards,
            h.bet,
            h.type_of_hand(true)
        )
    }

    let mut total_winning = 0;
    for (i, h) in hands.iter().enumerate() {
        total_winning += (i + 1) * h.bet
    }

    // too low: 245353894 -> 245794069
    total_winning
}

fn p1_card_to_value(c: &char) -> usize {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => panic!("invalid card"),
    }
}

fn p2_card_to_value(c: &char) -> usize {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        'J' => 1,
        _ => panic!("invalid card"),
    }
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<char>,
    bet: usize,
}

impl Hand {
    fn type_of_hand(&self, is_part_2: bool) -> usize {
        // 50 Five of a kind, where all five cards have the same label: AAAAA
        // 40 Four of a kind, where four cards have the same label and one card has a different label: AA8AA
        // 35 Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
        // 30 Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
        // 20 Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
        // 10 One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
        // 0 High card, where all cards' labels are distinct: 23456

        let mut counts: HashMap<usize, usize> = HashMap::new();
        for card in self.cards.iter() {
            let val = match is_part_2 {
                true => p2_card_to_value(card),
                false => p1_card_to_value(card),
            };
            match counts.get_mut(&val) {
                None => {
                    counts.insert(val, 1);
                }
                Some(v) => {
                    *v += 1;
                }
            }
        }

        if is_part_2 {
            // has J
            println!("p2_counts origin {:?}", counts);

            let has_j = counts.get_mut(&1).is_some();
            if has_j {
                let j_count = *counts.get_mut(&1).unwrap();
                // find max char count
                let (mut max_char, mut max_count) = (0, 0);
                for (k, v) in counts.iter() {
                    if *v > max_count {
                        if *k == 1 {
                            continue;
                        }
                        max_count = *v;
                        max_char = *k;
                        continue;
                    }
                }
                if max_char > 1 {
                    counts.remove(&1);
                    let max = counts.get_mut(&max_char).unwrap();
                    *max += j_count;
                }

                println!("p2_counts {:?}", counts);
            }
        }

        match counts.len() {
            1 => 50,
            2 => {
                // Four of a kind: 4 same char + 1 distinct
                // Full house: 3 same char + 2 same char
                let keys: Vec<&usize> = counts.keys().collect();
                let v = counts[keys[0]];
                if v == 1 || v == 4 {
                    return 40;
                }
                35
            }
            3 => {
                // 3 of a kind: 3 same char + 1 distinct + 1 distinct
                // 2 pair: 2 same char + 2 same char + 1 distinct
                for (_, v) in counts {
                    if v == 3 {
                        return 30;
                    }
                    if v == 2 {
                        return 20;
                    }
                }
                20
            }
            4 => 10,
            5 => 0,
            _ => {
                panic!("more than 5 counts")
            }
        }
    }
}

fn parse_input(file: &str) -> Vec<Hand> {
    let mut buffer = String::new();
    fs::File::open(file)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut hands: Vec<Hand> = Vec::new();
    for l in buffer.lines() {
        let data: Vec<&str> = l.trim().split(' ').collect();
        if data.len() != 2 {
            continue;
        }
        hands.push(Hand {
            cards: data[0].chars().collect(),
            bet: data[1].parse().unwrap(),
        })
    }

    hands
}
