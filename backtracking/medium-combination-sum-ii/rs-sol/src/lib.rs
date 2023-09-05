pub struct Solution {}

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();

        let mut result = Vec::<Vec<i32>>::new();
        let mut comb = Vec::<i32>::new();

        Self::dfs(&candidates, 0, &mut comb, target, &mut result);

        result
    }

    pub fn dfs(
        candidates: &Vec<i32>,
        mut idx: usize,
        comb: &mut Vec<i32>,
        target: i32,
        result: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            result.push(comb.clone());
            return;
        }

        if idx >= candidates.len() || target < 0 {
            return;
        }

        // add the current index
        while idx < candidates.len() - 1 && candidates[idx] == candidates[idx + 1] {
            idx += 1
        }

        let cand = candidates[idx];
        comb.push(cand);
        Self::dfs(candidates, idx + 1, comb, target - cand, result);
        comb.pop();

        // not add the current index
        Self::dfs(candidates, idx + 1, comb, target, result);
    }
}
