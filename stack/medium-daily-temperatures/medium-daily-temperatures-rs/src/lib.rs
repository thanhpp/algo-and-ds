// 56 ms - 5.2 MB
pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut resp = vec![0; temperatures.len()];

    let mut stack: Vec<usize> = Vec::new(); // temp, idx

    for i in 0..temperatures.len() {
        loop {
            let last = match stack.last() {
                None => {
                    break;
                }
                Some(x) => *x,
            };

            if resp[last] < temperatures[i] {
                stack.pop();
                resp[last] = (i - last) as i32;
                continue;
            }
            break;
        }

        stack.push(i);
    }

    resp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daily_temperatures() {
        let v1 = vec![73, 74, 75, 71, 69, 72, 76, 73];
        let v2 = vec![1, 1, 4, 2, 1, 1, 0, 0];
        assert_eq!(daily_temperatures(v1), v2);
    }
}
