pub struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let skip_first = Self::rob_seq(&nums[1..]);
        let skip_last = Self::rob_seq(&nums[..nums.len() - 1]);
        std::cmp::max(skip_first, skip_last)
    }

    fn rob_seq(nums: &[i32]) -> i32 {
        let (mut r_0, mut r_1) = (0, 0);

        for v in nums {
            // should rob this house
            let tmp = std::cmp::max(r_0 + *v, r_1);
            r_0 = r_1;
            r_1 = tmp;
        }

        r_1
    }
}
