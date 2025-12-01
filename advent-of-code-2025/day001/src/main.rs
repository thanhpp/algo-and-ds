fn main() {
    let data1 = include_str!("../input.1");
    let ins = read(data1);
    println!("ins count {}", &ins.len());

    // println!("solve 1: {}", solve(&ins, false));

    println!("solve 2: {}", solve(&ins, true));
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn read(data: &str) -> Vec<(Direction, i64)> {
    let mut instructions = vec![];
    for l in data.lines() {
        if l.is_empty() {
            continue;
        }

        let split = l.split_at(1);
        let dir = match split.0 {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => {
                panic!("unexpect direction {}", split.0);
            }
        };
        instructions.push((dir, split.1.parse().expect("parse count")));
    }

    instructions
}

fn solve(ins: &[(Direction, i64)], count_full_rotate: bool) -> i64 {
    const SIZE: i64 = 100;

    let mut count_0 = 0;
    let mut current = 50;
    for (d, v) in ins {
        let rotate = match d {
            Direction::Right => *v,
            Direction::Left => -v,
        };
        let simplified_rotate = rotate % SIZE;
        if count_full_rotate {
            count_0 += rotate.abs() / SIZE; // NOTE: rotate can be neg
        }

        let old = current;
        current += simplified_rotate;

        // NOTE: the condition for count_full_rotate skip start and end at 0
        if current < 0 {
            current += 100;
            if count_full_rotate && (current != 0 && old != 0) {
                count_0 += 1
            }
        } else if current > 99 {
            current -= 100;
            if count_full_rotate && (current != 0 && old != 0) {
                count_0 += 1
            }
        }

        if current == 0 {
            count_0 += 1;
        }

        println!("{:?}, current {}, count_0 {}", (d, v), current, count_0)
    }

    count_0
}
