package gosol

func rob(nums []int) int {
	if len(nums) == 1 {
		return nums[0]
	}

	// Can not rob the first house AND the last house
	// -> get the max profit of robbing except the first house
	// -> get the max profit of robbing except the last house

	return max(rob1(nums[1:]), rob1(nums[:len(nums)-1]))
}

// house robber 1 solution
func rob1(nums []int) int { // Time: O(n), Space: O(1)
	r1, r2 := 0, 0
	for _, n := range nums {
		r := max(r1+n, r2) // rob house[n] & house[n-2] || house[n-1]
		r1 = r2
		r2 = r
	}

	return r2
}

func max(a, b int) int {
	if a > b {
		return a
	}

	return b
}
