// 3 ms - 2.2 MB
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    if numbers.len() == 0 {
        return vec![];
    }

    let mut l = 0;
    let mut r = numbers.len() - 1;

    loop {
        if l >= r {
            break;
        }
        let tmp = numbers[l] + numbers[r];
        if tmp == target {
            return vec![(l + 1) as i32, (r + 1) as i32];
        }

        if tmp < target {
            l += 1;
            continue;
        }

        r -= 1;
        continue;
    }

    vec![]
}
