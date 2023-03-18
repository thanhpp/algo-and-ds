mod sol2;

struct Solution;

impl Solution {
    // 13 ms, 2.3 mb
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        // let mut sum = 0;
        let mut max_speed = 0;
        for p in piles.iter() {
            // sum += *p;
            if *p > max_speed {
                max_speed = *p
            }
        }

        let mut min_speed = 1;
        // if sum % h != 0 {
        //     min_speed += 1;
        // }

        loop {
            let m = (max_speed + min_speed) / 2;
            let eat_hour = Self::cal_eat_hours(&piles, m);
            println!(
                "min_speed {} max_speed {} eat_hour {} m {}",
                min_speed, max_speed, eat_hour, m
            );
            if eat_hour == h && max_speed - min_speed <= 1 {
                return m;
            }

            if max_speed - min_speed <= 1 {
                if Self::cal_eat_hours(&piles, min_speed) <= h {
                    return min_speed;
                }
                return max_speed;
            }

            if eat_hour > h {
                min_speed = m;
            } else {
                max_speed = m;
            }
        }
    }

    pub fn cal_eat_hours(piles: &Vec<i32>, k: i32) -> i32 {
        let mut eat_hours = 0;
        for p in piles.iter() {
            let t = *p / k;
            if *p % k != 0 {
                eat_hours += t + 1;
            } else {
                eat_hours += t;
            }
        }

        eat_hours
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_cal_eat_hours() {
        let h = super::Solution::cal_eat_hours(&vec![3, 6, 7, 11], 4);
        println!("{}", h);
    }

    #[test]
    fn test_min_eating_speed() {
        assert_eq!(super::Solution::min_eating_speed(vec![3, 6, 7, 11], 8), 4);
        println!("test done");
        assert_eq!(
            super::Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5),
            30
        );
        println!("test done");
        assert_eq!(
            super::Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6),
            23
        );
        println!("test done");
        assert_eq!(
            super::Solution::min_eating_speed(vec![312884470], 968709470),
            1
        );
        println!("test done");
        assert_eq!(
            super::Solution::min_eating_speed(vec![805306368, 805306368, 805306368], 1000000000),
            3
        );
    }
}
