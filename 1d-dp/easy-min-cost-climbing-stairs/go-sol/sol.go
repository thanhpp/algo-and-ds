package gosol

// BigO = O(n): 1 loop, visit each element only 1
func minCostClimbingStairs(cost []int) int {
	n0, n1 := 0, 0 // for the first 2 steps, no cost is needed

	// <= len(cost): must pass through the last step
	for i := 2; i <= len(cost); i++ { // O(n)
		// can be reached from either 2 previous steps
		// choose the minimum cost
		n0, n1 = n1, min(n0+cost[i-2], n1+cost[i-1])
	}

	return n1
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
