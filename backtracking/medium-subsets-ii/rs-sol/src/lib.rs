struct Solution {}

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut results = Vec::<Vec<i32>>::new();
        let mut subset = Vec::<i32>::new();

        Self::dfs(&nums, 0, &mut subset, &mut results);

        results
    }

    pub fn dfs(
        nums: &Vec<i32>,
        mut idx: usize,
        subset: &mut Vec<i32>,
        results: &mut Vec<Vec<i32>>,
    ) {
        if idx >= nums.len() {
            results.push(subset.clone());
            return;
        }

        // add the current idx to the subset
        subset.push(nums[idx]);
        Self::dfs(nums, idx + 1, subset, results);
        subset.pop();

        // find the index of the next unique value
        loop {
            if idx == nums.len() - 1 {
                break;
            }
            if nums[idx] != nums[idx + 1] {
                break;
            }

            idx += 1;
        }
        Self::dfs(nums, idx + 1, subset, results);
    }
}
