package gosol

/*
Concept: Building the longest sequence from index i (backwards)
- dp[i] = max(1, 1 + dp[j = i+1 -> len(nums)]) [cond: nums[j] > nums[i]]
*/
func lengthOfLIS(nums []int) int {
	/*
		Time = O(n ^ 2)
		Space = O(n)
	*/

	var lisFromIdx = make([]int, len(nums)) // Space O(n)

	for i := len(nums) - 1; i >= 0; i-- { // O(n)
		// default
		lisFromIdx[i] = 1
		// find max
		for j := i + 1; j < len(nums); j++ { // O(n)
			if nums[i] >= nums[j] {
				// not strictly increasing
				continue
			}

			lisFromIdx[i] = max(lisFromIdx[i], 1+lisFromIdx[j])
		}
	}

	return lisFromIdx[0]
}

func max(a, b int) int {
	if a > b {
		return a
	}

	return b
}
