struct Solution;

impl Solution {
    // pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    //     let (mut l, mut r): (i32, i32) = (1, *piles.iter().max().unwrap());
    //     let mut res = r;

    //     /*
    //     why l < r and r = m, instead of l <= r and r = m - 1?
    //     -> so we can account the value r = m?
    //     */
    //     // while l < r {
    //     //     let m = l + (r - l) / 2; // avoid l + r overflow
    //     //     let t: i32 = piles.iter().map(|&x| (x + m - 1) / m).sum();
    //     //     println!("{} {} {} {}", l, r, m, t);
    //     //     if t <= h {
    //     //         if m < res {
    //     //             res = m
    //     //         }
    //     //         r = m;
    //     //     } else {
    //     //         l = m + 1;
    //     //     }
    //     // }

    //     res
    // }

    // 12 ms, 2.4 MB
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let (mut l, mut r) = (1, *piles.iter().max().unwrap());
        let mut res = r;
        while l <= r {
            let m = (l + r) / 2;
            // why t need to be i64
            // the test case ([805306368, 805306368, 805306368], 1000000000)
            // can cause i32 overflow if we check with k = 1
            let t: i64 = piles.iter().map(|&x| ((x + m - 1) / m) as i64).sum();
            if t <= h as i64 {
                if m < res {
                    res = m
                }
                r = m - 1;
            } else {
                l = m + 1;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_min_eating_speed() {
        // assert_eq!(super::Solution::min_eating_speed(vec![3, 6, 7, 11], 8), 4);
        // println!("test done");
        // assert_eq!(
        //     super::Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5),
        //     30
        // );
        // println!("test done");
        // assert_eq!(
        //     super::Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6),
        //     23
        // );
        // println!("test done");
        // assert_eq!(
        //     super::Solution::min_eating_speed(vec![312884470], 968709470),
        //     1
        // );
        println!("test done");
        assert_eq!(
            super::Solution::min_eating_speed(vec![805306368, 805306368, 805306368], 1000000000),
            3
        );
        // 805_306_368
        // 1_000_000_000
    }
}
