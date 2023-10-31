package gosol

// BigO = O(n)
func rob(nums []int) int {
	if len(nums) < 2 {
		return nums[0] // only 1 house to rob :D
	}

	// max val of the first house = itself
	// max val of the second house = rob it || rob the first house
	h1, h2 := nums[0], max(nums[0], nums[1])
	for i := 2; i < len(nums); i++ { // O(n)
		h1, h2 = h2, max(h1+nums[i], h2) // rob it or not to rob it
	}

	return h2
}

func max(a, b int) int {
	if a > b {
		return a
	}

	return b
}
