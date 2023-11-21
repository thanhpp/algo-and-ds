package gosol

import "math"

/*
   Time = 2^n (each value choose or not choose)
*/

func lengthOfLISNaive(nums []int) int {
	var maxLength = 0
	for i := 0; i < len(nums); i++ {
		maxLength = max(maxLength, dfs(nums, i, math.MinInt, 0))
		// fmt.Println("----", i)
	}

	return maxLength
}

func dfs(nums []int, idx int, maxVal int, length int) int {
	// fmt.Println("idx", idx, "maxVal", maxVal, "length", length, "nums[idx]", nums[idx], "path", path)
	if idx >= len(nums) {
		// fmt.Println("idx >= len(nums)")
		return length
	}

	if nums[idx] <= maxVal {
		// fmt.Println("nums[idx] <= maxVal")
		return length
	}

	length++
	maxVal = nums[idx]
	var maxLength = length
	for i := idx + 1; i < len(nums); i++ {
		maxLength = max(maxLength, dfs(nums, i, maxVal, length))
	}

	return maxLength
}

func max(a, b int) int {
	if a > b {
		return a
	}

	return b
}
