package gosol

// BigO = O(n): iterate through each element once
func maxSubArray(nums []int) int {
	var (
		next   = 0
		curSum = 0
		maxSum = nums[0]
	)

	for next < len(nums) {
		if curSum <= 0 {
			// doesn't contribute if adding more number to it
			// reset the current sum
			curSum = 0
		}
		curSum += nums[next]
		maxSum = max(maxSum, curSum)
		next++ // O(n)
	}

	return maxSum
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
