package gosol

import "sort"

func combinationSum2(candidates []int, target int) [][]int {
	var (
		comb   []int
		result [][]int
		dfs    func(idx, target int)
	)

	// O(nlogn) - quicksort
	sort.Ints(candidates)

	// O(n * 2^n) - go through a tree height = len(candidates), width = len(candidates) * 2
	dfs = func(idx, target int) {
		if target == 0 {
			newComb := make([]int, len(comb))
			copy(newComb, comb)
			result = append(result, newComb)
			return
		}

		if target < 0 || idx >= len(candidates) || candidates[idx] > target {
			return
		}

		// add the current value
		cand := candidates[idx]
		comb = append(comb, cand)
		dfs(idx+1, target-cand)
		comb = comb[:len(comb)-1]

		// not add current value
		for ; idx < len(candidates)-1; idx++ {
			if candidates[idx] != candidates[idx+1] {
				break
			}
		}
		dfs(idx+1, target)
	}

	dfs(0, target)

	return result
}
