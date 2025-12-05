fn main() {
    let input = include_str!("../input.1");
    let (range, fresh) = read(input);

    println!("{:?}", solve_1(&range, &fresh));

    println!("{:?}", solve_2(&range));
}

fn read(s: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let mut range = s
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let split = l.split_once('-').unwrap();
            (split.0.parse().unwrap(), split.1.parse().unwrap())
        })
        .collect::<Vec<(i64, i64)>>();
    range.sort_by(|a, b| a.0.cmp(&b.0));

    let fresh = s
        .lines()
        .skip_while(|l| !l.is_empty())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect();

    (range, fresh)
}

fn solve_1(range: &[(i64, i64)], fresh: &[i64]) -> i64 {
    let mut count_1 = 0;
    let mut fresh_range_idx = vec![];
    for &f in fresh {
        for (i, &r) in range.iter().enumerate() {
            if f < r.0 {
                break;
            }
            if f <= r.1 {
                count_1 += 1;
                fresh_range_idx.push(i);
                break;
            }
        }
    }

    count_1
}

// solve_2 = merge overlap interval
fn solve_2(range: &[(i64, i64)]) -> i64 {
    if range.is_empty() {
        return 0;
    }

    let mut merged = Vec::<(i64, i64)>::new();
    merged.push(range[0]);

    let mut merged_idx = 0;
    for r in range.iter().skip(1) {
        if r.0 > merged[merged_idx].1 {
            merged.push(*r);
            merged_idx += 1;
            continue;
        }

        if r.1 <= merged[merged_idx].1 {
            continue;
        }

        merged[merged_idx].1 = r.1
    }

    // println!("{:?}", merged);

    merged.into_iter().fold(0, |acc, m| acc + m.1 - m.0 + 1)
}
