package gosol

func combinationSum(candidates []int, target int) [][]int {
	var (
		idx     = 0
		comb    = []int{}
		combSum = 0
		results [][]int
	)

	dfs(candidates, idx, target, comb, combSum, &results)

	return results
}

func dfs(candidates []int, idx int, target int, comb []int, combSum int, results *[][]int) {
	if idx >= len(candidates) {
		return
	}

	// add the current combination if sum == target
	if combSum == target {
		var newComb = make([]int, len(comb))
		copy(newComb, comb)
		*results = append(*results, newComb)

		return // can not add more
	}

	// stop if can not add more
	if combSum > target {
		return
	}

	// add the idx value
	comb = append(comb, candidates[idx])
	combSum += candidates[idx]

	// continue to add the current candidate
	dfs(candidates, idx, target, comb, combSum, results)

	// restore state
	comb = comb[:len(comb)-1]  // remove last added candidate
	combSum -= candidates[idx] // subtract added candidate

	// add the next candidate
	dfs(candidates, idx+1, target, comb, combSum, results)
}
