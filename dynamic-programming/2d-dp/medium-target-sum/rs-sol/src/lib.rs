pub struct Solution {}

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut cache = std::collections::HashMap::<(usize, i32), i32>::new();

        Self::dfs(&nums, target, &mut cache, 0, 0)
    }

    pub fn dfs(
        nums: &[i32],
        target: i32,
        cache: &mut std::collections::HashMap<(usize, i32), i32>,
        idx: usize,
        value: i32,
    ) -> i32 {
        if idx == nums.len() {
            if target == value {
                return 1;
            }
            return 0;
        }

        if let Some(v) = cache.get(&(idx, value)) {
            return *v;
        }

        let count = Self::dfs(nums, target, cache, idx + 1, value - nums[idx])
            + Self::dfs(nums, target, cache, idx + 1, value + nums[idx]);
        cache.insert((idx, value), count);

        count
    }
}
