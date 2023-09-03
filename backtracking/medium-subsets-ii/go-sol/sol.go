package gosol

import "sort"

func subsetsWithDup(nums []int) [][]int {
	var count = make(map[int]int) // num -> count
	for i := range nums {
		count[nums[i]] += 1
	}

	// must sort so all of the same values are placed into a group
	sort.Ints(nums)

	var results [][]int

	dfs(nums, 0, []int{}, &results)

	return results
}

func dfs(nums []int, idx int, sub []int, results *[][]int) {
	if idx >= len(nums) {
		tmp := make([]int, len(sub))
		copy(tmp, sub)
		*results = append(*results, tmp)
		return
	}

	// add the current value
	sub = append(sub, nums[idx])
	dfs(nums, idx+1, sub, results)
	sub = sub[:len(sub)-1]

	// not to add the current value
	for ; idx < len(nums)-1; idx++ { // find the last occurence that has the same value
		if nums[idx] != nums[idx+1] {
			break
		}
	}

	// add the next unique value
	dfs(nums, idx+1, sub, results)
}
