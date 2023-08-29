struct Solution {}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut exist = std::collections::HashSet::<i32>::new();
        let mut perm = Vec::<i32>::new();
        let mut res = Vec::<Vec<i32>>::new();

        Self::dfs(&nums, &mut exist, &mut perm, &mut res);

        res
    }

    fn dfs(
        nums: &Vec<i32>,
        exist: &mut std::collections::HashSet<i32>,
        perm: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if perm.len() == nums.len() {
            res.push(perm.clone());
            return;
        }

        for i in 0_usize..nums.len() {
            if exist.contains(&nums[i]) {
                continue;
            }

            perm.push(nums[i]);
            exist.insert(nums[i]);

            Self::dfs(nums, exist, perm, res);

            exist.remove(&nums[i]);
            perm.pop();
        }
    }
}
