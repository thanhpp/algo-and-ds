struct Solution {}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut comb = Vec::<i32>::new();
        let mut result = Vec::<Vec<i32>>::new();

        Self::dfs(&candidates, target, 0, &mut comb, &mut result);

        result
    }

    fn dfs(
        candidates: &Vec<i32>,
        target: i32,
        idx: usize,
        comb: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            res.push(comb.clone());
            return;
        }
        if target < 0 {
            return;
        }

        // try to add every candidates
        for i in idx..candidates.len() {
            comb.push(candidates[i]);
            Self::dfs(candidates, target - candidates[i], i, comb, res);
            comb.pop();
        }
    }
}
