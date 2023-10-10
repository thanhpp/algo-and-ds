package gosol

// BigO = O(n): 1 for loop
func climbStairs(n int) int {
	if n < 2 {
		return 1 // 1 step -> 1 way
	}

	n0, n1 := 1, 2           // 2 steps: first - 1; second - 1 + 1 & 2
	for i := 3; i < n; i++ { // O(n)
		// each step has 2 possible ways to reach it [i]
		// from the previous step [i - 1]
		// from the previous step of the previous step [i - 2]
		// pos(i) = pos[i - 1] + pos[i - 2]
		n0, n1 = n1, n0+n1
	}

	return n1
}
