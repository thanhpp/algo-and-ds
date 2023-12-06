use std::{fs, io::Read};

fn main() {
    const INPUT_FILE: &str = "input.txt";

    // p1: brute force
    let races = p1_parse_input(INPUT_FILE);
    println!("p1 races: {:#?}", races);
    println!("part_1: {}", part_1(&races));

    // p2
    let race = Race::parse_p2(INPUT_FILE);
    println!("p2 race: {:#?}", race);

    println!("part 2: {}", part_2(&race));
}

fn part_2(race: &Race) -> isize {
    /*
    speed = (holdDownTime)
    travelTime = (totalTime - holdDownTime)
    boatDistance = speed * travelTime
             = holdDownTime * (totalTime - holdDownTime)
             = holdownTime*totalTime - holdDownTime^2
    - In order to win, boatDistance > raceDistance
    => -holdDownTime^2 + totalTime*holdDownTime > raceDistance
    <=> -holdDownTime^2 + totalTime*holdDownTime - raceDistance > 0
    <=> holdDownTime^2 - totalTime*holdFownTime + raceDistance < 0
    <=> f(x) < 0
        - 2 cases
            - delta = 0 => x = -b/2a
            - delta > 0 => x1 < x < x2
            - delta < 0 => no answer
    *delta = b^2 - 4ac
    *x1, x2 = (-b - sqrt(delta))/2a, (-b + sqrt(delta))/2a
     */

    let time = race.time as f64;
    let distance = race.distance as f64;
    let delta = time.powi(2) - 4.0 * distance;

    if delta == 0.0 {
        if time / 2.0 > 1.0 {
            return 1;
        }
        return 0;
    }
    if delta < 0.0 {
        return 0;
    }

    let (x1, x2) = (
        (time - f64::sqrt(delta)) / 2.0,
        (time + f64::sqrt(delta)) / 2.0,
    );

    println!("x1: {}, x2: {}", x1, x2);

    (x2.floor() - x1.ceil()) as isize + 1
}

fn part_1(races: &[Race]) -> isize {
    let mut prod = 1;

    for r in races {
        // let mut win_count = 0;
        // for s in 1..r.time {
        //     if s * (r.time - s) > r.distance {
        //         win_count += 1
        //     }
        // }
        prod *= part_2(r)
    }

    prod
}

#[derive(Debug)]
pub struct Race {
    time: isize,
    distance: isize,
}

impl Race {
    fn parse_p2(file: &str) -> Race {
        let mut buffer = String::new();
        fs::File::open(file)
            .unwrap()
            .read_to_string(&mut buffer)
            .unwrap();
        let lines: Vec<&str> = buffer.lines().collect();
        if lines.len() != 2 {
            panic!("invalid lines len")
        };

        let time: isize = lines[0]
            .replace("Time:", "")
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect::<String>()
            .parse()
            .unwrap();

        let distance: isize = lines[1]
            .replace("Distance:", "")
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect::<String>()
            .parse()
            .unwrap();

        Race { time, distance }
    }
}

pub fn p1_parse_input(file: &str) -> Vec<Race> {
    let mut buffer = String::new();
    fs::File::open(file)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();
    let lines: Vec<&str> = buffer.lines().collect();

    if lines.len() != 2 {
        panic!("invalid lines len")
    };

    let times: Vec<_> = lines[0]
        .replace("Time:", "")
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    let distances: Vec<_> = lines[1]
        .replace("Distance:", "")
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    if times.len() != distances.len() {
        panic!("invalid distance")
    }

    let mut races = Vec::with_capacity(times.len());
    for i in 0..times.len() {
        races.push(Race {
            time: times[i],
            distance: distances[i],
        })
    }

    races
}
