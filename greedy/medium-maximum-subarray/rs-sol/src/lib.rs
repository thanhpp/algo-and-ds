pub struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = i32::MIN;
        let mut cur_sum = 0;

        for n in nums {
            cur_sum += n;
            max_sum = max_sum.max(cur_sum);

            if cur_sum <= 0 {
                cur_sum = 0
            }
        }

        max_sum
    }
}
