package gosol

// COMPLEXITY: 2^n (each num -> + or -)

func findTargetSumWaysDFS(nums []int, target int) int {
	var count = 0
	dfs(nums, 0, 1, target, &count)
	dfs(nums, 0, -1, target, &count)

	return count
}

func dfs(nums []int, idx int, mul int, target int, count *int) {
	if idx >= len(nums) {
		return
	}
	target -= mul * nums[idx]
	if target == 0 && idx == len(nums)-1 {
		*count++
		return
	}

	dfs(nums, idx+1, 1, target, count)
	dfs(nums, idx+1, -1, target, count)
}
