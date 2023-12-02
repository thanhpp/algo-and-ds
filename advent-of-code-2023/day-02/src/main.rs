use std::{fs, io::Read};

#[derive(Debug)]
pub struct Game {
    id: i32,
    sets: Vec<Set>,
}

#[derive(Debug)]
pub struct Set {
    red: i32,
    green: i32,
    blue: i32,
}

fn main() {
    let mut buffer = String::new();
    fs::File::open("input.txt")
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let games = parse_input(&buffer);

    println!("{:#?}", games);

    println!("{}", sum_1(&games));

    println!("{}", power_2(&games));
}

fn power_2(games: &Vec<Game>) -> i32 {
    let mut sum = 0;

    for g in games {
        let (mut max_red, mut max_green, mut max_blue) = (1, 1, 1); // set to 1 for power
        for s in g.sets.iter() {
            max_red = max_red.max(s.red);
            max_green = max_green.max(s.green);
            max_blue = max_blue.max(s.blue);
        }

        sum += max_red * max_blue * max_green;
    }

    sum
}

fn sum_1(games: &Vec<Game>) -> i32 {
    let (red_bound, green_bound, blue_bound) = (12, 13, 14);

    let mut sum: i32 = 0;

    for g in games {
        let mut valid = true;

        for set in g.sets.iter() {
            if set.red > red_bound || set.green > green_bound || set.blue > blue_bound {
                valid = false;
                break;
            }
        }

        if !valid {
            continue;
        }

        sum += g.id;
    }

    sum
}

fn parse_input(input: &str) -> Vec<Game> {
    let mut games = Vec::new();
    for l in input.split('\n') {
        let game_info: Vec<&str> = l.split(':').collect();
        if game_info.len() != 2 {
            panic!("invalid game_info length, {:#?}", game_info);
        }
        // game id
        let id: i32 = game_info[0].replace("Game ", "").parse().unwrap();

        // parse set
        let mut sets: Vec<Set> = Vec::new();
        let sets_info: Vec<&str> = game_info[1].split(';').collect();
        for s in sets_info {
            let mut set = Set {
                red: 0,
                green: 0,
                blue: 0,
            };
            let cubes: Vec<&str> = s.split(',').collect();

            for c in cubes {
                for color in ["red", "green", "blue"] {
                    if !c.contains(color) {
                        continue;
                    }

                    let count = c
                        .replace(color, "")
                        .trim()
                        .parse()
                        .expect("parse count error");

                    match color {
                        "red" => set.red = count,
                        "green" => set.green = count,
                        "blue" => set.blue = count,
                        _ => panic!("unknown color: {}", color),
                    }
                }
            }

            sets.push(set)
        }

        games.push(Game { id, sets });
    }

    games
}
