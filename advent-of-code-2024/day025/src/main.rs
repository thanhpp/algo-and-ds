fn main() {
    let input = include_str!("../input_1.txt");
    let prob = Problem::new(input);

    println!("size: {:?}", prob.size);

    solve1(&prob);
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub locks: Vec<Vec<Vec<char>>>,
    pub keys: Vec<Vec<Vec<char>>>,
    pub size: (i64, i64),
}

impl Problem {
    pub fn new(input: &str) -> Self {
        let input = format!("{}\n\n", input);
        let mut pins = Vec::<Vec<Vec<char>>>::new();
        let mut keys = Vec::<Vec<Vec<char>>>::new();
        let mut size = (0, 0);

        let mut current_map = Vec::<Vec<char>>::new();
        for l in input.lines() {
            if l.is_empty() {
                if current_map.is_empty() {
                    break;
                }
                size = (current_map.len(), current_map[0].len());

                let c = current_map[0].iter().filter(|&c| *c == '#').count();
                if c == current_map[0].len() {
                    pins.push(current_map.clone());
                } else {
                    keys.push(current_map.clone());
                }

                // reset
                current_map = vec![];
                continue;
            }
            current_map.push(l.chars().collect());
        }

        Problem {
            locks: pins,
            keys,
            size: (size.0 as i64, size.1 as i64),
        }
    }
}

fn to_height(is_lock: bool, data: &[Vec<char>]) -> Vec<i64> {
    let mut res = vec![0; data[0].len()];

    let data = match is_lock {
        true => data.iter().skip(1).collect::<Vec<_>>(),
        false => data.iter().rev().skip(1).collect::<Vec<_>>(),
    };

    for r in 0..data.len() {
        for c in 0..data[0].len() {
            if data[r][c] == '#' {
                res[c] += 1;
            }
        }
    }

    res
}

fn is_fit(key: &[i64], lock: &[i64], max_size: i64) -> bool {
    for i in 0..key.len() {
        if key[i] + lock[i] > max_size {
            return false;
        }
    }

    true
}

fn solve1(prob: &Problem) {
    let lock_heights = prob
        .locks
        .iter()
        .map(|m| to_height(true, m))
        .collect::<Vec<_>>();

    let key_heights = prob
        .keys
        .iter()
        .map(|m| to_height(false, m))
        .collect::<Vec<_>>();

    let mut res = 0;
    for k in key_heights.iter() {
        for l in lock_heights.iter() {
            if is_fit(k, l, prob.size.1) {
                res += 1;
            }
            println!("k: {:?}\nl: {:?}\nres: {}", k, l, res)
        }
    }

    println!("solve1: {}", res)
}
