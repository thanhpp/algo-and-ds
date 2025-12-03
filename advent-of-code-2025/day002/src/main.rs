fn main() {
    let input = read(include_str!("../input.1"));
    let solve_1 = input.iter().fold(0, |acc, v| acc + count_repeat(v.0, v.1));
    println!("solve 1 {}", solve_1);

    let solve_2 = input
        .iter()
        .fold(0, |acc, v| acc + count_repeat_2(v.0, v.1));
    println!("solve 2 {}", solve_2)
}

fn read(s: &str) -> Vec<(i64, i64)> {
    s.split(',')
        .filter(|s1| !s1.is_empty())
        .map(|s1| s1.split('-').collect::<Vec<&str>>())
        .map(|s2| (s2[0].parse().expect(s2[0]), s2[1].parse().expect(s2[1])))
        .collect()
}

fn count_repeat(start: i64, end: i64) -> i64 {
    let mut sum = 0;
    for i in start..=end {
        let str_i: Vec<u8> = i.to_string().bytes().collect();
        if !str_i.len().is_multiple_of(2) {
            continue;
        }

        let mid = str_i.len() / 2;
        let mut is_dup = true;
        for j in 0..mid {
            if str_i[j] != str_i[mid + j] {
                is_dup = false;
                break;
            }
        }
        if is_dup {
            sum += i
        }
    }

    sum
}

fn count_repeat_2(start: i64, end: i64) -> i64 {
    let mut sum = 0;
    for i in start..=end {
        let str_i: Vec<u8> = i.to_string().bytes().collect();
        // no duplicate
        if str_i.len() == 1 {
            continue;
        }

        let mut is_dup = false;
        for div in 2..=str_i.len() {
            if !str_i.len().is_multiple_of(div) {
                continue;
            }

            let d = str_i.len() / div;
            if d == 0 {
                // too long
                break;
            }

            // println!(
            //     "check: {}-{}, {}, {:?}, {}",
            //     start,
            //     end,
            //     i,
            //     &str_i[0..d],
            //     div,
            // );

            if str_i[0..d].repeat(div).eq(&str_i) {
                is_dup = true;
                break;
            }
        }

        if is_dup {
            sum += i;
            // println!("dup: {}-{}, {}", start, end, i)
        }
    }

    sum
}
