fn main() {
    let input = include_str!("../input.1");
    let data = read(input);
    println!("solve1 {}", solve_1(&data));
    println!("solve2 {}", solve_2(input));
}

#[derive(Debug)]
pub enum Operation {
    Add,
    Mul,
}

fn read(s: &str) -> Vec<(Vec<&str>, Operation)> {
    let data: Vec<Vec<&str>> = s.lines().map(|l| l.split_whitespace().collect()).collect();
    if data.is_empty() {
        panic!("empty data")
    }

    let (r, c) = (data.len(), data[0].len());
    let mut res = Vec::<(Vec<&str>, Operation)>::new();
    for cur_c in 0..c {
        let mut split = Vec::<&str>::new();
        for cur_r in 0..r - 1 {
            split.push(data[cur_r][cur_c]);
        }
        let op = match data[r - 1][cur_c] {
            "+" => Operation::Add,
            "*" => Operation::Mul,
            _ => panic!("invalid op {}", data[r - 1][cur_c]),
        };

        res.push((split, op));
    }

    res
}

fn solve_1(input: &[(Vec<&str>, Operation)]) -> i64 {
    input
        .iter()
        .map(|(v, op)| match op {
            Operation::Add => v.iter().map(|s| s.parse::<i64>().unwrap()).sum::<i64>(),
            Operation::Mul => v.iter().map(|s| s.parse::<i64>().unwrap()).product(),
        })
        .sum()
}

fn solve_2(input: &str) -> i64 {
    let lines = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();
    if lines.is_empty() {
        return 0;
    }

    let (max_r, max_c) = (lines.len(), lines[0].len());
    let mut parsed: Vec<Vec<Vec<char>>> = vec![vec![]; max_r];

    // scan by cols
    let mut init = true;
    let mut cur_num = 0;
    for c in 0..max_c {
        let mut all_white_space = true;
        for r in 0..max_r {
            if init {
                parsed[r].push(vec![]);
            }
            if !lines[r][c].is_whitespace() {
                all_white_space = false
            }
            parsed[r][cur_num].push(lines[r][c]);
        }
        init = false;
        if all_white_space {
            init = true;
            cur_num += 1;
        }
    }
    for p in parsed.iter() {
        println!("{:?}", p)
    }
    // remove last white space from [0..max_r][0..max_c-1]
    // max_c -1 => no last white space
    let (max_r, max_c) = (parsed.len(), parsed[0].len());
    for r in 0..max_r {
        for c in 0..max_c - 1 {
            parsed[r][c] = parsed[r][c][0..parsed[r][c].len() - 1].to_vec();
        }
    }
    for p in parsed.iter() {
        println!("{:?}", p)
    }

    let mut rotated: Vec<Vec<Vec<char>>> = vec![];
    for c in 0..max_c {
        let mut rt = vec![];
        for r in 0..max_r {
            rt.push(parsed[r][c].clone());
        }
        rotated.push(rt);
    }

    let mut ins = Vec::<(Vec<i64>, Operation)>::new();
    for r in rotated.iter() {
        ins.push(to_nums(r));
        println!("--")
    }

    ins.iter()
        .map(|(v, op)| match op {
            Operation::Add => v.iter().sum::<i64>(),
            Operation::Mul => v.iter().product(),
        })
        .sum()
}

fn to_nums(chars: &[Vec<char>]) -> (Vec<i64>, Operation) {
    let (max_r, max_c) = (chars.len(), chars[0].len());

    let mut res = vec![];
    let mut op = Operation::Add;
    for c in 0..max_c {
        let mut nums = 0;
        let mut pow = 0;
        for r in (0..max_r).rev() {
            if !chars[r][c].is_ascii_digit() {
                match chars[r][c] {
                    '+' => {
                        op = Operation::Add;
                    }
                    '*' => {
                        op = Operation::Mul;
                    }
                    _ => {}
                }
                continue;
            }
            nums += chars[r][c].to_digit(10).unwrap() as i64 * 10_i64.pow(pow);
            pow += 1;

            // println!("{} {} {}", r, c, chars[r][c])
        }
        println!("{nums}");
        res.push(nums);
    }

    (res, op)
}
