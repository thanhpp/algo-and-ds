package gosol

func combinationSum2(candidates []int, target int) [][]int {
	var (
		comb    = []int{}
		results [][]int
	)

	var fn func(remains int, idx int)
	fn = func(remains int, idx int) {
		if remains == target {
			tmp := make([]int, len(comb))
			copy(tmp, comb)
			results = append(results, tmp)
			return
		}
		if remains < 0 {
			return
		}

		for i := idx; i < len(candidates); i++ {
			comb = append(comb, candidates[i])
			fn(remains-candidates[i], i)
			comb = comb[:len(comb)-1]
		}
	}

	fn(target, 0)

	return results
}
