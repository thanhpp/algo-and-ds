pub struct Solution {}

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max = i32::MIN;
        let (mut curr_max, mut curr_min) = (1, 1);

        for n in nums {
            // println!("{} {} {} {}", n, max, curr_max, curr_min);
            if n == 0 {
                curr_max = 1;
                curr_min = 1;
                max = max.max(0);
                continue;
            }

            let (tmp_max, tmp_min) = (curr_max * n, curr_min * n);
            curr_max = tmp_max.max(tmp_min).max(n);
            curr_min = tmp_max.min(tmp_min).min(n);
            max = max.max(curr_max);
        }

        max
    }
}
