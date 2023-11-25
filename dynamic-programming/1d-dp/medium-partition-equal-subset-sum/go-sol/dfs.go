package gosol

func canPartitionDFS(nums []int) bool {
	// Time: O(2^n) -> with each number, there are 2 options: add or not add

	var sum int
	for _, n := range nums {
		sum += n
	}

	return dfs(nums, 0, 0, sum)
}

func dfs(nums []int, idx int, currentSum int, leftSum int) bool {
	if currentSum == leftSum {
		return true
	}

	if currentSum > leftSum {
		return false
	}

	if idx >= len(nums) {
		return false
	}

	return dfs(nums, idx+1, currentSum+nums[idx], leftSum-nums[idx]) || dfs(nums, idx+1, currentSum, leftSum)
}
