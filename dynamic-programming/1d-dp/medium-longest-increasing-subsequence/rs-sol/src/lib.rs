pub struct Solution {}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut max_length = 0;
        let mut lis_from_idx = vec![1; nums.len()];

        for i in (0..nums.len()).rev() {
            for j in i + 1..nums.len() {
                if nums[i] >= nums[j] {
                    continue;
                }
                lis_from_idx[i] = lis_from_idx[i].max(lis_from_idx[j] + 1);
            }

            max_length = max_length.max(lis_from_idx[i])
        }

        max_length
    }
}
