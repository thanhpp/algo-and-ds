fn main() {
    let data = include_str!("../input.1");
    let input = read(data);
    println!("solve_1 {}", solve_1(&input));

    println!("solve_2 {}", solve_2(&input));
}

fn read(s: &str) -> Vec<Vec<i64>> {
    s.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i64).collect())
        .collect()
}

fn solve_1(banks: &[Vec<i64>]) -> i64 {
    let mut sum = 0;
    for b in banks {
        sum += find_largest_bat(b);
    }

    sum
}

fn solve_2(banks: &[Vec<i64>]) -> i64 {
    let mut sum = 0;
    for b in banks {
        sum += find_largest_bat_2(b);
    }

    sum
}

// find_largest_bat: find largest x[i], x[i + 1]
fn find_largest_bat(bats: &[i64]) -> i64 {
    let (mut max_1, mut res) = (0, 0);
    for i in 0..bats.len() {
        if bats[i] < max_1 {
            continue;
        }
        max_1 = bats[i];
        let mut max_2 = 0;
        for j in i + 1..bats.len() {
            if bats[j] < max_2 {
                continue;
            }
            max_2 = bats[j];
            res = res.max(max_1 * 10 + max_2);
        }
    }

    res
}

// find_largest_bat_2: because we need 12 digits,
// each digit range from 0-9
// for every number: if number1[i] > number2[i] => number1 > number2 (number1[0..i] == number2[0..i])
// => to form the largest number, we need to find the largest digits in possible range
// possible range: for a 12 digits number, we need to reserve enough range (when choosing the 1st digit -> need reserve 11 digits)
fn find_largest_bat_2(bats: &[i64]) -> i64 {
    let mut max = 0;
    let mut start_at = 0;
    for i in (0..12).rev() {
        let mut tmp = 0;
        let start = start_at;
        for i1 in start..bats.len() - i {
            if bats[i1] <= tmp {
                continue;
            }
            tmp = bats[i1];
            start_at = i1 + 1;
        }

        max += tmp * 10_i64.pow((i) as u32);
    }

    // println!("{:?} {}", bats, max);

    max
}
