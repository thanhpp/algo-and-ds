package gosol

func permute(nums []int) [][]int {
	var fac = 1
	for i := 2; i <= len(nums); i++ {
		fac *= i
	}
	var (
		results = make([][]int, 0, fac)
		exist   = make(map[int]struct{}, len(nums))
	)

	var fn func(perm []int)
	fn = func(perm []int) {
		// add current permutation
		if len(perm) >= len(nums) {
			tmp := make([]int, len(perm))
			copy(tmp, perm)
			results = append(results, tmp)
			return
		}

		for i := 0; i < len(nums); i++ {
			// exist -> skip
			if _, ok := exist[nums[i]]; ok {
				continue
			}

			// add current
			perm = append(perm, nums[i])
			exist[nums[i]] = struct{}{}

			// add next
			fn(perm)

			// reset
			delete(exist, nums[i])
			perm = perm[:len(perm)-1]
		}
	}
	fn(make([]int, 0, len(nums)))

	return results
}
