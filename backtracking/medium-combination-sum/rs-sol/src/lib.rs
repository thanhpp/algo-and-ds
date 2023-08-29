mod lib2;

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut results = Vec::<Vec<i32>>::new();
    let comb = Vec::<i32>::new();
    let comb_sum = 0;

    self::dfs(&candidates, 0, comb, comb_sum, target, &mut results);

    results
}

fn dfs(
    candidates: &Vec<i32>,
    idx: usize,
    mut comb: Vec<i32>,
    mut comb_sum: i32,
    target: i32,
    results: &mut Vec<Vec<i32>>,
) {
    // can not have any other combination
    if idx >= candidates.len() || comb_sum > target {
        return;
    }

    // take the current combination
    if comb_sum == target {
        results.push(comb.clone());
        return;
    }

    let candidate = candidates[idx];

    // add the current idx
    comb.push(candidate);
    comb_sum += candidate;
    self::dfs(candidates, idx, comb.clone(), comb_sum, target, results);

    // revert state
    comb.pop();
    comb_sum -= candidate;
    // try add the next idx
    self::dfs(candidates, idx + 1, comb, comb_sum, target, results);
}
