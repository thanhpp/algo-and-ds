pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut pairs: Vec<(i32, i32)> = Vec::new(); // (pos, speed)
    for i in 0..position.len() {
        pairs.push((position[i], speed[i]));
    }

    // sort by position O(nlogn)
    pairs.sort_by(|a, b| a.0.cmp(&b.0));

    // stack is used to store the time a car will get to the target
    // if the time to target of a car < time to target of an existing car
    // -> these 2 cars met in between (pos[existing car], target)
    let mut stack: Vec<f64> = Vec::new(); // time to target
    for i in (0..pairs.len()).rev() {
        let time_to_target: f64 = (target - pairs[i].0) as f64 / pairs[i].1 as f64;
        match stack.last() {
            None => {
                stack.push(time_to_target);
            }
            Some(t) => {
                if *t < time_to_target {
                    // the current car cannot catch up the car in the stack
                    stack.push(time_to_target)
                }
            }
        }
    }

    stack.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_car_fleet() {
        assert_eq!(car_fleet(100, vec![0, 2, 4], vec![4, 2, 1]), 1);
        assert_eq!(car_fleet(10, vec![3], vec![3]), 1);
        assert_eq!(car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]), 3);
    }
}
