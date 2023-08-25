pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut results: Vec<Vec<i32>> = vec![];

    self::dfs(&nums, 0, vec![], &mut results);

    results
}

pub fn dfs(nums: &Vec<i32>, idx: usize, mut subset: Vec<i32>, results: &mut Vec<Vec<i32>>) {
    if idx == nums.len() {
        results.push(subset);
        return;
    }

    let idx = idx + 1;
    dfs(nums, idx, subset.clone(), results);

    subset.push(nums[idx]);
    dfs(nums, idx, subset, results)
}
